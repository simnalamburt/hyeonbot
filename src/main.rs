// Copyright 2015-2026 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::io::Write;
use std::process::ExitCode;

use hyeonbot::Settings;
use tracing_subscriber::EnvFilter;

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
        "{LOGO}\nhyeonbot\n\n  version  : v{}\n  Rust     : {}\n  Platform : {}\n  Profile  : {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("HYEONBOT_RUSTC_VERSION"),
        env!("HYEONBOT_TARGET"),
        env!("HYEONBOT_PROFILE"),
    );
    let _ = std::io::stdout().flush();

    // Load configs from environment variables
    let settings = Settings {
        server: env::var("HYEONBOT_SERVER").unwrap_or_else(|_| "irc.ozinger.org".to_owned()),
        port: env::var("HYEONBOT_PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or(6697),
        use_tls: true,
        db_path: "db".into(),
    };
    let log_level = match env::var("HYEONBOT_LOG_LEVEL").as_deref() {
        // Cinch (Ruby) log level names that tracing does not know
        Ok("log") => "info".to_owned(),
        Ok("fatal") => "error".to_owned(),
        Ok(level) => level.to_owned(),
        Err(_) => "debug".to_owned(),
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
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
