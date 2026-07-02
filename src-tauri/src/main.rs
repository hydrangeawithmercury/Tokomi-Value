// Tokomi Value — native shell
// Responsibilities kept deliberately small and stable:
//   1. Host the (already-tested) HTML/JS analyzer in an always-on-top window.
//   2. Register a global shortcut to summon / dismiss the window.
//   3. Expose a command so the UI can toggle "always on top" on demand.
//
// Screen capture itself stays in the webview (getDisplayMedia) for this first
// build — that path is proven. Native window-capture-by-name is a later step.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Toggle the main window's always-on-top flag from the UI.
#[tauri::command]
fn set_always_on_top(window: tauri::WebviewWindow, on: bool) {
    let _ = window.set_always_on_top(on);
}

/// Show/hide the main window (used by the tray-less "summon" shortcut).
fn toggle_visibility(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        match win.is_visible() {
            Ok(true) => {
                let _ = win.hide();
            }
            _ => {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }
    }
}

fn main() {
    // Ctrl + Shift + V  →  summon / dismiss
    let summon = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
    let summon_for_handler = summon.clone();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state == ShortcutState::Pressed && shortcut == &summon_for_handler {
                        toggle_visibility(app);
                    }
                })
                .build(),
        )
        .setup(move |app| {
            app.global_shortcut().register(summon)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_always_on_top])
        .run(tauri::generate_context!())
        .expect("error while running Tokomi Value");
}
