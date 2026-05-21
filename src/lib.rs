//! Local validation harness for the example WorkConductor extension.
//!
//! The real hub source lives in `example_extension.rs` at the repository root.
//! This crate is only a local compile/test adapter: it exposes a minimal copy of
//! WorkConductor's extension trait surface at `crate::traits` so the same file
//! compiles here and after WorkConductor copies it into `agixt-extensions`
//! during image builds.

pub mod traits {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::collections::HashMap;

    pub trait Extension: Send + Sync {
        fn name(&self) -> String;

        fn description(&self) -> String {
            String::new()
        }

        fn category(&self) -> String {
            "uncategorized".to_string()
        }

        fn commands(&self) -> Vec<CommandMetadata>;

        fn settings(&self) -> Vec<SettingMetadata> {
            Vec::new()
        }

        fn init(&mut self, _settings: &HashMap<String, String>) -> anyhow::Result<()> {
            Ok(())
        }

        fn execute(&self, command: &str, args: &HashMap<String, Value>) -> anyhow::Result<Value>;

        fn is_ready(&self) -> bool {
            true
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CommandMetadata {
        pub name: String,
        pub description: String,
        #[serde(default)]
        pub arguments: Vec<ArgumentMetadata>,
        #[serde(default)]
        pub returns: Option<String>,
        #[serde(default)]
        pub requires_auth: bool,
    }

    impl CommandMetadata {
        pub fn new(name: &str, description: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
                arguments: Vec::new(),
                returns: None,
                requires_auth: false,
            }
        }

        pub fn with_argument(mut self, arg: ArgumentMetadata) -> Self {
            self.arguments.push(arg);
            self
        }

        pub fn with_returns(mut self, returns: &str) -> Self {
            self.returns = Some(returns.to_string());
            self
        }

        pub fn requires_auth(mut self) -> Self {
            self.requires_auth = true;
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ArgumentMetadata {
        pub name: String,
        pub description: String,
        #[serde(rename = "type")]
        pub arg_type: String,
        #[serde(default)]
        pub required: bool,
        #[serde(default)]
        pub default: Option<Value>,
    }

    impl ArgumentMetadata {
        pub fn new(name: &str, description: &str, arg_type: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
                arg_type: arg_type.to_string(),
                required: false,
                default: None,
            }
        }

        pub fn required(mut self) -> Self {
            self.required = true;
            self
        }

        pub fn with_default(mut self, default: Value) -> Self {
            self.default = Some(default);
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SettingMetadata {
        pub name: String,
        pub description: String,
        #[serde(rename = "type")]
        pub setting_type: String,
        #[serde(default)]
        pub required: bool,
        #[serde(default)]
        pub default: Option<String>,
        #[serde(default)]
        pub secret: bool,
    }

    impl SettingMetadata {
        pub fn new(name: &str, description: &str, setting_type: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
                setting_type: setting_type.to_string(),
                required: false,
                default: None,
                secret: false,
            }
        }

        pub fn required(mut self) -> Self {
            self.required = true;
            self
        }

        pub fn with_default(mut self, default: &str) -> Self {
            self.default = Some(default.to_string());
            self
        }

        pub fn secret(mut self) -> Self {
            self.secret = true;
            self
        }
    }
}

#[path = "../example_extension.rs"]
pub mod example_extension;

pub use example_extension::{ExampleExtension, ExampleItem};
