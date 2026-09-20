//! Minimal community wasm plugin. WIT guest.

wit_bindgen::generate!({
    world: "plugin",
    path: "plugin.wit",
});

use crate::exports::tianshu::plugin::guest::Guest;
use crate::tianshu::plugin::host;

pub const PLUGIN_ID: &str = "nostalgiatan.ping";
pub const PLUGIN_JSON: &str = include_str!("../plugin.json");
pub const UI_JSON: &str = include_str!("../ui.json");

struct Component;

export!(Component);

impl Guest for Component {
    fn abi_version() -> u32 {
        3
    }

    fn id() -> String {
        PLUGIN_ID.into()
    }

    fn ui_json() -> String {
        UI_JSON.into()
    }

    fn step(kind: String, params_json: String) -> String {
        if kind != "ping" {
            return String::new();
        }
        let params: serde_json::Value = serde_json::from_str(&params_json)
            .unwrap_or(serde_json::Value::Object(Default::default()));
        let mut text = params
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        if text.is_empty() {
            text = ir_text(&host::read("in", "in", ""));
        }
        format!("Ping: {text}")
    }

    fn on_tool(id: String) -> String {
        let Ok(ui) = serde_json::from_str::<serde_json::Value>(UI_JSON) else {
            return String::new();
        };
        let Some(tools) = ui.get("tools").and_then(|v| v.as_array()) else {
            return String::new();
        };
        for t in tools {
            if t.get("id").and_then(|v| v.as_str()) != Some(id.as_str()) {
                continue;
            }
            let op = t.get("op").and_then(|v| v.as_str()).unwrap_or("");
            let args = t.get("args").cloned().unwrap_or(serde_json::json!({}));
            return host::apply(op, &args.to_string());
        }
        String::new()
    }
}

fn ir_text(raw: &str) -> String {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(raw) else {
        return raw.to_string();
    };
    match v {
        serde_json::Value::Object(m) => match m.get("type").and_then(|t| t.as_str()) {
            Some("String") => m
                .get("value")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            Some("Number") => m.get("value").map(|x| x.to_string()).unwrap_or_default(),
            _ => raw.to_string(),
        },
        serde_json::Value::String(s) => s,
        _ => raw.to_string(),
    }
}
