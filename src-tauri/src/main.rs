#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, RunEvent};
use std::process::{Command, Stdio};

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Wyjdź");

  let tray_menu = SystemTrayMenu::new()
      .add_item(quit);

  let system_tray = SystemTray::new()
    .with_menu(tray_menu);


  let app = tauri::Builder::default()
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
          SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
          } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
          }

          SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
              "quit" => {
                std::process::exit(0);
              }
              _ => {}
            }
          }
          _ => {}
        })
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

    if cfg!(windows) {
      match Command::new("posnet-server.exe")
          .stdin(Stdio::piped())
          .stderr(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn() {
              Ok(output) => {
                  println!("{:#?}", output);
                  match output.wait_with_output() {
                      Ok(wait_out) => println!("{:#?}", wait_out),
                      Err(err) => println!("{:#?}", err),
                  }
              },
              Err(err) => println!("{:#?}", err),
      }
  }

  app.run(|app_handle, e| match e {
    RunEvent::CloseRequested { label, api, .. } => {
        let window = app_handle.get_window(&label).unwrap();
        api.prevent_close();
        window.minimize().unwrap();
    }

    RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  })
}
