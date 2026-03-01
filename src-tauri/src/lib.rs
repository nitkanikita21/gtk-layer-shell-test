use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tauri::Manager;

pub mod modules;

const PANEL_HEIGHT: i32 = 40;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      #[cfg(target_os = "linux")]
      {
        let window = app.get_webview_window("main").unwrap();
        window.open_devtools();
        let gtk_window = window.gtk_window().unwrap();

        gtk_window.init_layer_shell();
        gtk_window.set_layer(Layer::Top);
        gtk_window.set_anchor(Edge::Top, true);
        gtk_window.set_anchor(Edge::Left, true);
        gtk_window.set_anchor(Edge::Right, true);
        gtk_window.set_namespace(Some("panel"));

        // використовуємо константу
        gtk_window.set_default_height(PANEL_HEIGHT);
        gtk_window.set_exclusive_zone(PANEL_HEIGHT);

        let provider = gtk4::CssProvider::new();
        provider.load_from_data("window { background-color: transparent; }");
        gtk_window
          .style_context()
          .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        window.with_webview(|webview| {
          use gtk4::{gdk::RGBA, prelude::*};
          use webkit6::{prelude::WebViewExt, WebView};

          let webview: WebView = unsafe {
            webview
              .inner()
              .clone()
              .downcast::<WebView>()
              .expect("Not a WebKit WebView")
          };

          webview.set_hexpand(true);
          webview.set_vexpand(false);
          webview.set_height_request(PANEL_HEIGHT);
          webview.set_background_color(&RGBA::new(0.0, 0.0, 0.0, 0.0));
        })?;

        gtk_window.show();
      }

      let app_handle = app.handle().clone();
      tauri::async_runtime::block_on(async {
        tokio::spawn(modules::system::start_background_tasks());
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      greet,
      modules::system::get_system_info,
      modules::media::get_media_info,
      modules::media::media_play_pause,
      modules::media::media_next,
      modules::media::media_previous,
      modules::media::media_stop,
      modules::media::media_set_volume,
      modules::media::media_toggle_play_pause
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
