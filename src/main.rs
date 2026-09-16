use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use hyeonbot::Settings;
use serde::Deserialize;
use tracing_subscriber::EnvFilter;

/// Contents of `config.toml`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    /// IRC server host
    server: String,
    /// IRC server port
    #[serde(default = "default_port")]
    port: u16,
    /// Log level or filter directives (`trace`, `debug`, `info`, `warn`, `error`)
    #[serde(default = "default_log_level")]
    log_level: String,
}

fn default_port() -> u16 {
    6697
}

fn default_log_level() -> String {
    "debug".to_owned()
}

fn load_config(path: &Path) -> Result<Config, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    toml::from_str(&text).map_err(|err| format!("failed to parse {}: {err}", path.display()))
}

const LOGO: &str = r"
                  .........
             ....................
         ...........................
       ...............................
      .................................
    .....=###-..........................
   .....+####*...........:###+...........
  ......-####:..........:%####-..........
 ........................#####...........
 ...........:.............-+:............
............#=..=#=......................
............:#-#+:#=..**.................
..............-....*#*...................
.......+**=..............................
.....=*:..==#=...........................
...:+:.......*:..........................
...:........==..........................
  .........=+.........................
      .............................
";

#[tokio::main]
async fn main() -> ExitCode {
    println!(
        "{LOGO}\nhyeonbot\n\n  version  : v{}\n  Platform : {}-{}\n  Profile  : {}\n",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::ARCH,
        std::env::consts::OS,
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    );
    let _ = std::io::stdout().flush();

    // Load configs from config.toml (or the file given as the first argument)
    let path = std::env::args_os()
        .nth(1)
        .map_or_else(|| "config.toml".into(), std::path::PathBuf::from);
    let config = match load_config(&path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };
    let settings = Settings {
        server: config.server,
        port: config.port,
        use_tls: true,
        db_path: "db".into(),
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(config.log_level))
        .init();

    tokio::select! {
        result = hyeonbot::run(settings) => {
            // `run` only returns when the database is unusable
            tracing::error!("{}", result.unwrap_err());
            ExitCode::FAILURE
        }
        // Handle ^C gracefully
        _ = tokio::signal::ctrl_c() => {
            println!("\n\nBye!\n\n");
            ExitCode::SUCCESS
        }
    }
}
