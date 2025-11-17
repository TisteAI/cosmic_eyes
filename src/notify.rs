//! Desktop notifications via D-Bus

use zbus::{proxy, Connection};

/// D-Bus proxy for Notifications interface
#[proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    /// Send a notification
    /// Returns notification ID
    async fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

/// Send a desktop notification
pub async fn send_notification(title: &str, body: &str) {
    if let Err(e) = try_send_notification(title, body).await {
        // Log error but don't fail - notifications are optional
        eprintln!("Failed to send notification: {}", e);
    }
}

async fn try_send_notification(title: &str, body: &str) -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    proxy
        .notify(
            "Cosmic Eyes",
            0,
            "cosmic-eyes",
            title,
            body,
            &[],
            std::collections::HashMap::new(),
            5000, // 5 seconds
        )
        .await?;

    Ok(())
}
