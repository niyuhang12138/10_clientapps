mod commands;
mod utils;

use commands::get_app_dir;
use commands::greet;
use tauri::{
    webview::PageLoadPayload, App, Builder, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent, Wry,
};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Target, TargetKind};
use tracing::debug;
use tracing::info;
use utils::log_dir;

pub fn app() -> anyhow::Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_deep_link::init())
        // .plugin(tauri_plugin_window_state::Builder::new().build())
        // .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir,])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler);

    Ok(builder)
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");
    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::new().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[allow(unused_assignments)]
    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .title("Hacker News")
            .inner_size(1200_f64, 800_f64)
            .min_inner_size(800_f64, 600_f64)
            .resizable(true)
            .content_protected(true);
    }

    let web_view = builder.build()?;

    #[cfg(debug_assertions)]
    web_view.open_devtools();

    // initialize updater...

    Ok(())
}

// on_page_load is called when the webview is create
// signature should be: Fn(&Webview<R>, &PageLoadPayload<'_>) + Send + Sync + 'static
fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page load loaded on {:?}", webview.label())
}

// on_window_event is called when the window event is triggered
// signature should be: Fn(&Window<R>, &WindowEvent) + Send + Sync + 'static
fn window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        info!("Window Close event: {:?}", window.label());
        if window.label() == "min" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: Some("app".to_string()),
            }),
            Target::new(TargetKind::Stdout),
        ])
        .with_colors(ColoredLevelConfig::default())
        .level(tracing::log::LevelFilter::Info)
}
