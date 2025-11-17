use clap::{Parser, Subcommand};
use zbus::{proxy, Connection};

/// D-Bus service name
const SERVICE_NAME: &str = "com.github.cosmiceyes";
/// D-Bus object path
const OBJECT_PATH: &str = "/com/github/cosmiceyes";

/// D-Bus proxy for timer interface
#[proxy(
    interface = "com.github.cosmiceyes.Timer",
    default_service = "com.github.cosmiceyes",
    default_path = "/com/github/cosmiceyes"
)]
trait Timer {
    async fn start_break(&self, break_type: &str) -> zbus::Result<()>;
    async fn skip_break(&self) -> zbus::Result<()>;
    async fn postpone_break(&self, break_type: &str) -> zbus::Result<()>;
    async fn pause(&self) -> zbus::Result<()>;
    async fn resume(&self) -> zbus::Result<()>;
    async fn get_status(&self) -> zbus::Result<(String, i64, i64)>;
}

/// CLI interface for Cosmic Eyes break reminder
#[derive(Parser)]
#[command(name = "cosmic-eyes-cli")]
#[command(about = "Control the Cosmic Eyes break reminder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a break immediately
    Break {
        /// Type of break: "short" or "long"
        #[arg(value_parser = ["short", "long"])]
        break_type: String,
    },

    /// Skip the current break
    Skip,

    /// Postpone the next break
    Postpone {
        /// Type of break to postpone: "short" or "long"
        #[arg(value_parser = ["short", "long"])]
        break_type: String,
    },

    /// Show current status
    Status,

    /// Pause the timer
    Pause,

    /// Resume the timer
    Resume,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Connect to D-Bus and create proxy
    let connection = match Connection::session().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to D-Bus: {}", e);
            eprintln!("Make sure the cosmic-eyes applet is running.");
            std::process::exit(1);
        }
    };

    let proxy = match TimerProxy::new(&connection).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to connect to cosmic-eyes service: {}", e);
            eprintln!("Make sure the cosmic-eyes applet is running.");
            std::process::exit(1);
        }
    };

    // Execute command
    let result = match cli.command {
        Commands::Break { break_type } => {
            proxy.start_break(&break_type).await
                .map(|_| format!("Started {} break", break_type))
        }
        Commands::Skip => {
            proxy.skip_break().await
                .map(|_| "Skipped current break".to_string())
        }
        Commands::Postpone { break_type } => {
            proxy.postpone_break(&break_type).await
                .map(|_| format!("Postponed {} break", break_type))
        }
        Commands::Status => {
            match proxy.get_status().await {
                Ok((state, short_secs, long_secs)) => {
                    let state_display = match state.as_str() {
                        "Running" => "Active",
                        "Paused" => "Paused",
                        "InBreak:Short" => "In short break",
                        "InBreak:Long" => "In long break",
                        "Postponed" => "Break postponed",
                        _ => &state,
                    };

                    let short_display = format_duration(short_secs);
                    let long_display = format_duration(long_secs);

                    Ok(format!(
                        "Status: {}\nNext short break: {}\nNext long break: {}",
                        state_display, short_display, long_display
                    ))
                }
                Err(e) => Err(e),
            }
        }
        Commands::Pause => {
            proxy.pause().await
                .map(|_| "Paused timer".to_string())
        }
        Commands::Resume => {
            proxy.resume().await
                .map(|_| "Resumed timer".to_string())
        }
    };

    // Print result or error
    match result {
        Ok(msg) => println!("{}", msg),
        Err(e) => {
            eprintln!("Command failed: {}", e);
            std::process::exit(1);
        }
    }
}

/// Format seconds into human-readable duration
fn format_duration(seconds: i64) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;

    if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
