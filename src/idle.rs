// ! Idle detection via D-Bus ScreenSaver interface

use zbus::{proxy, Connection};

/// D-Bus proxy for ScreenSaver interface (KDE/GNOME compatible)
#[proxy(
    interface = "org.freedesktop.ScreenSaver",
    default_service = "org.freedesktop.ScreenSaver",
    default_path = "/org/freedesktop/ScreenSaver"
)]
trait ScreenSaver {
    /// Get idle time in milliseconds
    async fn get_session_idle_time(&self) -> zbus::Result<u32>;
}

/// Check if system is idle
pub async fn is_idle(threshold_seconds: u64) -> bool {
    // Try to connect and check idle time
    match check_idle_time(threshold_seconds).await {
        Ok(is_idle) => is_idle,
        Err(_) => {
            // If D-Bus check fails, assume not idle
            // This prevents false pauses if screensaver service isn't available
            false
        }
    }
}

async fn check_idle_time(threshold_seconds: u64) -> zbus::Result<bool> {
    let connection = Connection::session().await?;
    let proxy = ScreenSaverProxy::new(&connection).await?;

    let idle_ms = proxy.get_session_idle_time().await?;
    let idle_seconds = idle_ms as u64 / 1000;

    Ok(idle_seconds >= threshold_seconds)
}
