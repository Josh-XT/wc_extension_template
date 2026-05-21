/* Example WorkConductor desktop extension.
 *
 * This UI talks to the Rust backend through WorkConductor's generic command
 * execution endpoint:
 *
 *   POST /v1/extensions/run
 *
 * The matching Rust command module lives in example_extension.rs and is
 * registered by workconductor.toml during WorkConductor image builds.
 */
window.AgixtRegisterExtension('example_extension', {
  mount(container, ctx) {
    const view = new ExampleExtensionView(container, ctx);
    container._exampleExtensionView = view;
    view.start();
  },
  unmount() {
    const root = document.querySelector(
      '.chat-screen-main .view-pane[data-view="example_extension"]'
    );
    if (root && root._exampleExtensionView) {
      root._exampleExtensionView.stop();
      root._exampleExtensionView = null;
    }
  },
});

function ExampleExtensionView(container, ctx) {
  this.container = container;
  this.ctx = ctx;
  this.items = [];
  this.scopes = new Set();
  this.roleId = null;
  this.error = null;
  this.loading = false;
}

ExampleExtensionView.prototype.start = function () {
  this.render();
  this.bootstrap();
};

ExampleExtensionView.prototype.stop = function () {
  this.container.innerHTML = '';
};

ExampleExtensionView.prototype.fetchJson = async function (path, opts) {
  const url = new URL(path, this.ctx.serverUrl).toString();
  const init = {
    method: (opts && opts.method) || 'GET',
    headers: Object.assign(
      { Authorization: 'Bearer ' + this.ctx.jwt },
      opts && opts.json != null ? { 'Content-Type': 'application/json' } : {}
    ),
  };
  if (opts && opts.json != null) init.body = JSON.stringify(opts.json);

  const resp = await fetch(url, init);
  if (resp.status === 204) return null;
  const text = await resp.text();
  let data = null;
  try {
    data = text ? JSON.parse(text) : null;
  } catch (_) {
    /* non-JSON response */
  }
  if (!resp.ok) {
    const err = new Error((data && data.detail) || 'HTTP ' + resp.status);
    err.status = resp.status;
    throw err;
  }
  return data;
};

ExampleExtensionView.prototype.runCommand = async function (commandName, commandArgs) {
  const data = await this.fetchJson('/v1/extensions/run', {
    method: 'POST',
    json: {
      command_name: commandName,
      command_args: commandArgs || {},
      agent_name: this.ctx.agentName || 'XT',
      conversation_name: this.ctx.conversationId || '',
    },
  });
  const response = data && Object.prototype.hasOwnProperty.call(data, 'response')
    ? data.response
    : data;
  if (typeof response === 'string') {
    try {
      return JSON.parse(response);
    } catch (_) {
      return { success: true, response };
    }
  }
  return response || {};
};

ExampleExtensionView.prototype.bootstrap = async function () {
  await this.loadUserAndScopes();
  await this.refresh();
};

ExampleExtensionView.prototype.loadUserAndScopes = async function () {
  try {
    const user = await this.fetchJson('/v1/user');
    const company =
      (user.companies || []).find((candidate) => candidate.id === this.ctx.companyId) ||
      (user.companies || [])[0];
    if (company) {
      this.scopes = new Set(company.scopes || []);
      this.roleId = company.role_id != null ? company.role_id : null;
    }
  } catch (_) {
    /* keep rendering; command calls remain backend-authoritative */
  }
};

ExampleExtensionView.prototype.hasScopeAction = function (action) {
  if (this.roleId === 0 || this.roleId === 1) return true;
  const prefix = 'ext:example_extension:';
  for (const scope of this.scopes) {
    if (scope === prefix + action) return true;
    if (scope.startsWith(prefix) && scope.endsWith(':' + action)) return true;
  }
  return false;
};

ExampleExtensionView.prototype.refresh = async function () {
  this.loading = true;
  this.renderBody();
  try {
    const data = await this.runCommand('List Example Items', {});
    this.items = Array.isArray(data.items) ? data.items : [];
    this.error = null;
  } catch (err) {
    this.error = err;
  } finally {
    this.loading = false;
    this.renderBody();
  }
};

ExampleExtensionView.prototype.render = function () {
  this.container.innerHTML = `
    <section class="example-extension-ui" style="padding:16px;display:flex;flex-direction:column;gap:12px;">
      <header style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
        <h1 style="margin:0;font-size:18px;">Example Extension</h1>
        <button type="button" data-action="refresh">Refresh</button>
        <button type="button" data-action="create">New Item</button>
      </header>
      <div data-region="error" style="color:#c44;"></div>
      <div data-region="body">Loading...</div>
    </section>
  `;
  this.container
    .querySelector('[data-action="refresh"]')
    .addEventListener('click', () => this.refresh());
  this.container
    .querySelector('[data-action="create"]')
    .addEventListener('click', () => this.openCreate());
};

ExampleExtensionView.prototype.renderBody = function () {
  const body = this.container.querySelector('[data-region="body"]');
  const errEl = this.container.querySelector('[data-region="error"]');
  if (!body || !errEl) return;

  errEl.textContent = this.error ? this.error.message : '';
  if (this.loading) {
    body.textContent = 'Loading...';
    return;
  }
  if (!this.items.length) {
    body.textContent = 'No items yet.';
    return;
  }

  const rows = this.items
    .map((item) => {
      const del = this.hasScopeAction('delete')
        ? `<button type="button" data-delete="${escapeAttr(item.id)}">Delete</button>`
        : '';
      return `
        <tr>
          <td>${escapeHtml(item.name || '')}</td>
          <td>${escapeHtml(item.description || '')}</td>
          <td>${escapeHtml(item.status || '')}</td>
          <td>${del}</td>
        </tr>
      `;
    })
    .join('');

  body.innerHTML = `
    <table style="width:100%;border-collapse:collapse;">
      <thead>
        <tr>
          <th align="left">Name</th>
          <th align="left">Description</th>
          <th align="left">Status</th>
          <th></th>
        </tr>
      </thead>
      <tbody>${rows}</tbody>
    </table>
  `;
  body.querySelectorAll('[data-delete]').forEach((button) => {
    button.addEventListener('click', () => this.handleDelete(button.dataset.delete));
  });
};

ExampleExtensionView.prototype.openCreate = async function () {
  if (!this.hasScopeAction('write')) {
    this.error = new Error('You do not have permission to create items.');
    this.renderBody();
    return;
  }
  const name = prompt('Name?');
  if (!name) return;
  const description = prompt('Description?') || '';

  try {
    await this.runCommand('Create Example Item', {
      name,
      description,
    });
    await this.refresh();
  } catch (err) {
    this.error = err;
    this.renderBody();
  }
};

ExampleExtensionView.prototype.handleDelete = async function (itemId) {
  if (!itemId) return;
  try {
    await this.runCommand('Delete Example Item', { item_id: itemId });
    await this.refresh();
  } catch (err) {
    this.error = err;
    this.renderBody();
  }
};

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

function escapeAttr(value) {
  return escapeHtml(value).replace(/`/g, '&#096;');
}
