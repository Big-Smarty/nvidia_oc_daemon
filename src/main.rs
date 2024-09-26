mod config;
#[macro_use]
extern crate log;
extern crate simplelog;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use simplelog::*;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

static CONFIG: &str = "/etc/nvidia_oc/config";

fn main() -> ! {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("demon.log").unwrap(),
        ),
    ])
    .unwrap();

    let config_dir = Path::new(CONFIG)
        .parent()
        .unwrap_or(Path::new("/etc/nvidia_oc"));
    let mut config = config::Config::init();
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let tx_clone = tx.clone();

    let mut watcher: RecommendedWatcher =
        notify::recommended_watcher(move |res: notify::Result<Event>| match res {
            Ok(event) => {
                if let EventKind::Modify(_) = event.kind {
                    trace!("File changed");
                    if let Err(err) = tx_clone.send(Ok(event)) {
                        error!("Error sending event: {:?}", err);
                    }
                }
            }
            Err(e) => {
                error!("Error while looking for config file update: {e}")
            }
        })
        .unwrap();

    watcher.watch(config_dir, RecursiveMode::Recursive).unwrap();
    config.apply();

    let mut last_apply = Instant::now();
    let debounce_duration = Duration::from_secs(5); // Adjust this value as needed

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => {
                let now = Instant::now();
                if now.duration_since(last_apply) >= debounce_duration {
                    trace!("Debounce period passed, applying config");
                    config = config::Config::from_config();
                    config.apply();
                    last_apply = now;
                } else {
                    trace!("Ignoring event due to debounce");
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                trace!("Timeout; intended behaviour");
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                error!("Disconnected; make sure the sender isn't dropped");
            }
        }
    }
}

