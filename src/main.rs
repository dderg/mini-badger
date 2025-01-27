mod app_monitor;
mod config;

use tray_icon::{TrayIconBuilder, Icon};
use winit::event_loop::{EventLoop, ControlFlow};
use app_monitor::{MonitorManager, CustomEvent};
use config::Config;
use std::path::PathBuf;

fn load_icon_for_app(app_name: &str, config: &Config) -> Result<Icon, Box<dyn std::error::Error>> {
    // 1. Check for custom icon in config
    if let Some(custom_path) = config.apps.get(app_name).and_then(|app| app.icon_path.as_ref()) {
        if let Ok(icon) = load_icon_from_path(custom_path) {
            return Ok(icon);
        }
    }

    // 2. Check bundled icons
    let icon_path = format!("../assets/icons/{}.png", app_name.to_lowercase());
    if let Ok(icon) = load_bundled_icon(&icon_path) {
        return Ok(icon);
    }

    // 3. Fallback to default icon
    let icon = load_bundled_icon("../assets/default-icon.png")
        .expect("Default icon must exist");
    Ok(icon)
}

fn load_icon_from_path(path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();
    Icon::from_rgba(rgba.into_raw(), width, height)
        .map_err(|e| e.into())
}

fn load_bundled_icon(relative_path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    // Convert the icon bytes into an image::DynamicImage
    let img = match relative_path {
        "../assets/icons/things.png" => image::load_from_memory(include_bytes!("../assets/icons/things.png"))?,
        "../assets/icons/mail.png" => image::load_from_memory(include_bytes!("../assets/icons/mail.png"))?,
        "../assets/default-icon.png" => image::load_from_memory(include_bytes!("../assets/default-icon.png"))?,
        _ => return Err("Icon not found".into()),
    };

    // Convert to RGBA format
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();
    
    // Create tray icon from RGBA data
    Icon::from_rgba(rgba.into_raw(), width, height)
        .map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop: EventLoop<CustomEvent> = EventLoop::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    
    // Load configuration
    let config = Config::load();
    
    let mut monitor_manager = MonitorManager::new();
    
    // Get first app for reference and make it owned
    let first_app = config.apps.keys()
        .next()
        .expect("No apps configured")
        .to_string();
    
    // Get initial count
    let initial_count = app_monitor::AppMonitor::new(first_app.clone(), 2).get_count();
    // Try to load icon in this order:
    // 1. Custom icon from config
    // 2. Bundled icon
    // 3. Default fallback icon
    let icon = load_icon_for_app(&first_app, &config)?;

    let tray_icon = TrayIconBuilder::new()
    .with_tooltip("Mini-Badger")
    .with_title(&initial_count)
    .with_icon(icon)
    .build()?;

    // Configure monitors based on config file
    for (app_name, app_config) in &config.apps {
        monitor_manager.add_monitor(
            app_name.clone(), 
            app_config.interval_secs,
            event_loop_proxy.clone()
        );
    }

    // Move owned first_app into the event loop
    event_loop.run(move |event, _, control_flow| {
        // Set the control flow to wait for events
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::UserEvent(CustomEvent::UpdateCount(app_name, count)) => {
                if app_name == *first_app {
                    tray_icon.set_title(Some(&count));
                }
            }
            winit::event::Event::NewEvents(winit::event::StartCause::Init) => {
                // Initial setup if needed
            }
            _ => (),
        }
    })
}
