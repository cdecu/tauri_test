// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(dead_code)]
mod errors;
mod events;
mod ipc;
mod perm_controller;

use events::{Events, EventsImpl};
use ipc::{RootApi, RootApiImpl};
use perm_controller::LineCodec;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;

/// The state of the application.
#[derive(Clone)]
pub struct AppState {
  app_handle: Option<tauri::AppHandle>,
  _serial_port: Option<String>,
  app_id: i32,
}

impl AppState {
  pub fn new() -> Self {
    Self {
      app_handle: None,
      _serial_port: None,
      app_id: 123,
    }
  }
}

type GlobalAppState = Arc<Mutex<AppState>>;

/// The main function of the application.
#[tokio::main]
async fn main() {
  let state = Arc::new(Mutex::new(AppState::new()));

  let router = taurpc::Router::new()
    .merge(RootApiImpl { state: state.clone() }.into_handler())
    .merge(EventsImpl { _state: state.clone() }.into_handler());

  // let tty_path = "/tmp/gt_com";
  let tty_path = "/dev/pts/8";
  // let mut port = tokio_serial::new(tty_path, 9600)
  //   .open_native_async()
  //   .expect("Unable to open port");

  #[cfg(unix)]
  // port
  //   .set_exclusive(false)
  //   .expect("Unable to set serial port exclusive to false");

  // let mut reader = LineCodec.framed(port);

  tauri::Builder::default()
    .invoke_handler(router.into_handler())
    .setup(move |app| {
      let app_handle = app.handle();
      tokio::spawn(async move {
        let mut state = state.lock().await;
        state.app_handle = Some(app_handle);
        state.app_id = 456;
        // while let Some(line_result) = reader.next().await {
        //     let line = line_result.expect("Failed to read line");
        //     println!("{}", line);
        // }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
