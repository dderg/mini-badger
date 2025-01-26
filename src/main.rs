mod app_monitor;
mod config;

use tray_icon::TrayIconBuilder;
use winit::event_loop::{EventLoop, ControlFlow};
use app_monitor::{MonitorManager, CustomEvent};
use config::Config;

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

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("Mini-Badger")
        .with_title(&initial_count)
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
