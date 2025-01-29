mod app_monitor;
mod config;

use tray_icon::{TrayIconBuilder, Icon};
use winit::event_loop::{EventLoop, ControlFlow};
#[cfg(target_os = "macos")]
use {
    cocoa::appkit::{NSApplication, NSApplicationActivationPolicy},
    objc::{msg_send, sel, sel_impl, class},
};
use app_monitor::{MonitorManager, CustomEvent};
use config::Config;
use std::path::PathBuf;

fn load_icon_for_app(app_name: &str, config: &Config) -> Result<Icon, Box<dyn std::error::Error>> {
    // 1. Check for custom icon in config
    if let Some(custom_path) = config.apps.get(app_name).and_then(|app| app.icon_path.as_ref()) {
        match load_icon_from_path(custom_path) {
            Ok(icon) => return Ok(icon),
            Err(e) => eprintln!("Failed to load custom icon for {}: {}", app_name, e),
        }
    }

    // 2. Fallback to default icon
    let icon = load_bundled_icon()
        .expect("Default icon must exist");
    Ok(icon)
}

fn load_icon_from_path(path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let expanded_path = if path.starts_with('~') {
        dirs::home_dir()
            .ok_or("Could not find home directory")?
            .join(&path[2..])
    } else {
        PathBuf::from(path)
    };

    let img = image::open(expanded_path)?;
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();
    Icon::from_rgba(rgba.into_raw(), width, height)
        .map_err(|e| e.into())
}

fn load_bundled_icon() -> Result<Icon, Box<dyn std::error::Error>> {
    // Convert the icon bytes into an image::DynamicImage
    let img = image::load_from_memory(include_bytes!("../assets/default-icon.png"))?;

    // Convert to RGBA format
    let rgba = img.into_rgba8();
    let (width, height) = rgba.dimensions();
    
    // Create tray icon from RGBA data
    Icon::from_rgba(rgba.into_raw(), width, height)
        .map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hide from dock on macOS
    #[cfg(target_os = "macos")]
    unsafe {
        let app: cocoa::base::id = msg_send![class!(NSApplication), sharedApplication];
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
    }

    let event_loop: EventLoop<CustomEvent> = EventLoop::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    
    // Load configuration
    let config = Config::load();

    let mut monitor_manager = MonitorManager::new();
    
    // Create a HashMap to store all tray icons
    use std::collections::HashMap;
    let mut tray_icons: HashMap<String, tray_icon::TrayIcon> = HashMap::new();

    // Create a tray icon for each app in config
    for (app_name, app_config) in &config.apps {
        // Get initial count for this app
        let initial_count = app_monitor::AppMonitor::new(app_name.clone(), 2).get_count();
        
        // Load icon for this app
        let icon = load_icon_for_app(app_name, &config)?;

        // Create tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip(&format!("Mini-Badger - {}", app_name))
            .with_title(&initial_count)
            .with_icon(icon)
            .with_icon_as_template(true)
            .build()?;

        // Store tray icon
        tray_icons.insert(app_name.clone(), tray_icon);

        // Configure monitor
        monitor_manager.add_monitor(
            app_name.clone(),
            app_config.interval_secs,
            event_loop_proxy.clone()
        );
    }

    // Move tray_icons into the event loop
    event_loop.run(move |event, _, control_flow| {
        // Set the control flow to wait for events
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::UserEvent(CustomEvent::UpdateCount(app_name, count)) => {
                if let Some(tray_icon) = tray_icons.get(&app_name) {
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
