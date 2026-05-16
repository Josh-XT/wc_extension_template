//! Example WorkConductor extension module.
//!
//! This module is intentionally small and self-contained so an agent can turn it
//! into a real WorkConductor Rust extension. WorkConductor currently registers
//! Rust command extensions as compiled modules in `agixt-extensions`; desktop UI
//! bundles still live under `desktop/<id>/` and call the Rust backend.

use crate::traits::{ArgumentMetadata, CommandMetadata, Extension, SettingMetadata};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

const EXTENSION_NAME: &str = "example_extension";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ExampleExtension {
    items: Mutex<HashMap<String, ExampleItem>>,
    greeting: String,
}

impl ExampleExtension {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
            greeting: "Hello from a WorkConductor Rust extension.".to_string(),
        }
    }

    fn items(&self) -> Result<MutexGuard<'_, HashMap<String, ExampleItem>>> {
        self.items
            .lock()
            .map_err(|_| anyhow!("example item store lock was poisoned"))
    }

    fn required_string(args: &HashMap<String, Value>, key: &str) -> Result<String> {
        args.get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .ok_or_else(|| anyhow!("{key} is required"))
    }

    fn optional_string(args: &HashMap<String, Value>, key: &str) -> Option<String> {
        args.get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
    }

    fn create_item(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let now = Utc::now();
        let item = ExampleItem {
            id: uuid::Uuid::new_v4().to_string(),
            name: Self::required_string(args, "name")?,
            description: Self::optional_string(args, "description"),
            status: Self::optional_string(args, "status").unwrap_or_else(|| "active".to_string()),
            created_at: now,
            updated_at: now,
        };

        self.items()?.insert(item.id.clone(), item.clone());

        Ok(json!({
            "success": true,
            "message": self.greeting,
            "item": item,
        }))
    }

    fn list_items(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let status = Self::optional_string(args, "status").map(|value| value.to_lowercase());
        let search = Self::optional_string(args, "search").map(|value| value.to_lowercase());
        let mut items = self
            .items()?
            .values()
            .filter(|item| {
                if let Some(status) = status.as_deref() {
                    if item.status.to_lowercase() != status {
                        return false;
                    }
                }
                if let Some(search) = search.as_deref() {
                    let haystack = format!(
                        "{} {}",
                        item.name,
                        item.description.clone().unwrap_or_default()
                    )
                    .to_lowercase();
                    if !haystack.contains(search) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect::<Vec<_>>();

        items.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));

        Ok(json!({
            "success": true,
            "items": items,
            "count": items.len(),
        }))
    }

    fn get_item(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let item_id = Self::required_string(args, "item_id")?;
        let item = self
            .items()?
            .get(&item_id)
            .cloned()
            .ok_or_else(|| anyhow!("Item not found"))?;

        Ok(json!({
            "success": true,
            "item": item,
        }))
    }

    fn update_item(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let item_id = Self::required_string(args, "item_id")?;
        let mut items = self.items()?;
        let item = items
            .get_mut(&item_id)
            .ok_or_else(|| anyhow!("Item not found"))?;

        if let Some(name) = Self::optional_string(args, "name") {
            item.name = name;
        }
        if args.contains_key("description") {
            item.description = Self::optional_string(args, "description");
        }
        if let Some(status) = Self::optional_string(args, "status") {
            item.status = status;
        }
        item.updated_at = Utc::now();

        Ok(json!({
            "success": true,
            "item": item.clone(),
        }))
    }

    fn delete_item(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let item_id = Self::required_string(args, "item_id")?;
        let deleted = self.items()?.remove(&item_id).is_some();

        Ok(json!({
            "success": deleted,
            "deleted": deleted,
            "item_id": item_id,
        }))
    }
}

impl Default for ExampleExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl Extension for ExampleExtension {
    fn name(&self) -> String {
        EXTENSION_NAME.to_string()
    }

    fn description(&self) -> String {
        "Example WorkConductor Rust extension with CRUD-style commands.".to_string()
    }

    fn category(&self) -> String {
        "Core Abilities".to_string()
    }

    fn commands(&self) -> Vec<CommandMetadata> {
        vec![
            CommandMetadata::new("Create Example Item", "Create a new example item")
                .with_argument(ArgumentMetadata::new("name", "Item name", "string").required())
                .with_argument(ArgumentMetadata::new(
                    "description",
                    "Optional description",
                    "string",
                ))
                .with_argument(ArgumentMetadata::new(
                    "status",
                    "Optional status, defaults to active",
                    "string",
                )),
            CommandMetadata::new("List Example Items", "List example items")
                .with_argument(ArgumentMetadata::new(
                    "status",
                    "Filter by status",
                    "string",
                ))
                .with_argument(ArgumentMetadata::new(
                    "search",
                    "Search name/description",
                    "string",
                )),
            CommandMetadata::new("Get Example Item", "Get one example item")
                .with_argument(ArgumentMetadata::new("item_id", "Item ID", "string").required()),
            CommandMetadata::new("Update Example Item", "Update an example item")
                .with_argument(ArgumentMetadata::new("item_id", "Item ID", "string").required())
                .with_argument(ArgumentMetadata::new("name", "New item name", "string"))
                .with_argument(ArgumentMetadata::new(
                    "description",
                    "New description",
                    "string",
                ))
                .with_argument(ArgumentMetadata::new("status", "New status", "string")),
            CommandMetadata::new("Delete Example Item", "Delete an example item")
                .with_argument(ArgumentMetadata::new("item_id", "Item ID", "string").required()),
        ]
    }

    fn settings(&self) -> Vec<SettingMetadata> {
        vec![SettingMetadata::new(
            "EXAMPLE_EXTENSION_GREETING",
            "Greeting included in create responses",
            "string",
        )
        .with_default("Hello from a WorkConductor Rust extension.")]
    }

    fn init(&mut self, settings: &HashMap<String, String>) -> Result<()> {
        if let Some(greeting) = settings
            .get("EXAMPLE_EXTENSION_GREETING")
            .map(String::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            self.greeting = greeting.to_string();
        }
        Ok(())
    }

    fn execute(&self, command: &str, args: &HashMap<String, Value>) -> Result<Value> {
        match command {
            "Create Example Item" => self.create_item(args),
            "List Example Items" => self.list_items(args),
            "Get Example Item" => self.get_item(args),
            "Update Example Item" => self.update_item(args),
            "Delete Example Item" => self.delete_item(args),
            other => Err(anyhow!("Unknown command: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[(&str, Value)]) -> HashMap<String, Value> {
        values
            .iter()
            .map(|(key, value)| ((*key).to_string(), value.clone()))
            .collect()
    }

    #[test]
    fn create_list_update_delete_round_trip() {
        let extension = ExampleExtension::new();

        let created = extension
            .execute(
                "Create Example Item",
                &args(&[
                    ("name", json!("Apples")),
                    ("description", json!("A test item")),
                ]),
            )
            .unwrap();
        let item_id = created["item"]["id"].as_str().unwrap().to_string();

        let listed = extension
            .execute("List Example Items", &HashMap::new())
            .unwrap();
        assert_eq!(listed["count"], json!(1));

        let updated = extension
            .execute(
                "Update Example Item",
                &args(&[("item_id", json!(item_id)), ("status", json!("done"))]),
            )
            .unwrap();
        assert_eq!(updated["item"]["status"], json!("done"));

        let deleted = extension
            .execute(
                "Delete Example Item",
                &args(&[("item_id", updated["item"]["id"].clone())]),
            )
            .unwrap();
        assert_eq!(deleted["deleted"], json!(true));
    }
}
