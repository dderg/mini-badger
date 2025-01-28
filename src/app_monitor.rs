use std::process::Command;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use winit::event_loop::EventLoopProxy;

pub enum CustomEvent {
    UpdateCount(String, String), // (app_name, count)
}

pub struct AppMonitor {
    name: String,
    interval: Duration,
    last_count: Arc<Mutex<String>>,
}

impl AppMonitor {
    pub fn new(name: String, interval_secs: u64) -> Self {
        Self {
            name,
            interval: Duration::from_secs(interval_secs),
            last_count: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn get_count(&self) -> String {
        let find_output = Command::new("lsappinfo")
            .args(["find", &format!("LSDisplayName={}", self.name)])
            .output()
            .expect("Failed to execute find command");

        let app_id = String::from_utf8_lossy(&find_output.stdout).trim().to_string();

        let info_output = Command::new("lsappinfo")
            .args(["info", "-only", "StatusLabel", &app_id])
            .output()
            .expect("Failed to execute info command");

        let output_str = String::from_utf8_lossy(&info_output.stdout);
        let label = output_str
            .split("\"label\"=\"")
            .nth(1)
            .and_then(|s| s.split('\"').next())
            .unwrap_or("0")
            .to_string();

        label
    }

    pub fn start_monitoring(&self, event_loop: EventLoopProxy<CustomEvent>) {
        let name = self.name.clone();
        let interval = self.interval;
        let last_count = self.last_count.clone();
        
        std::thread::spawn(move || {
            let monitor = AppMonitor::new(name.clone(), interval.as_secs());
            loop {
                let count = monitor.get_count();
                
                // Only send an event if the count has changed
                {
                    let mut last = last_count.lock().unwrap();
                    if *last != count {
                        *last = count.clone();
                        // Send event to the event loop
                        let _ = event_loop.send_event(CustomEvent::UpdateCount(name.clone(), count));
                    }
                }
                
                std::thread::sleep(interval);
            }
        });
    }
}

pub struct MonitorManager {
    _monitors: Vec<AppMonitor>,
}

impl MonitorManager {
    pub fn new() -> Self {
        Self {
            _monitors: Vec::new(),
        }
    }

    pub fn add_monitor(&mut self, name: String, interval_secs: u64, event_loop: EventLoopProxy<CustomEvent>) {
        let monitor = AppMonitor {
            name,
            interval: Duration::from_secs(interval_secs),
            last_count: Arc::new(Mutex::new(String::new())),
        };
        monitor.start_monitoring(event_loop);
        self._monitors.push(monitor);
    }
}
