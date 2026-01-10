use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct ActiveWindow {
    class: Option<String>,
    title: Option<String>,
    pid: Option<i32>,
}

pub fn get_active_window() -> Option<String> {
    let output = std::process::Command::new("hyprctl")
        .args(["activewindow", "-j"])
        .output()
        .ok()?;

    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).ok()?;

    let class = value.get("class")?.as_str()?;
    Some(
        class
            .rsplit('.')
            .next()
            .unwrap_or(class)
            .to_string(),
    )
}
