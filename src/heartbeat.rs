//! Emits a heartbeat internal metric.
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use libsystemd::daemon::{self, NotifyState};

use tokio::time::interval;

use crate::internal_events::Heartbeat;

/// Emits Heartbeat event every second.
pub async fn heartbeat() {
    let since = Instant::now();
    let mut interval = interval(Duration::from_secs(1));

    #[cfg(target_os = "linux")]
    let _systemd_timeout = daemon::watchdog_enabled(true).expect("Watchdog disabled");

    loop {
        #[cfg(target_os = "linux")]
        let _sent = daemon::notify(false, &[NotifyState::Watchdog])
            .expect("systemd watchdog notify failed");

        interval.tick().await;
        emit!(Heartbeat { since });
    }
}
