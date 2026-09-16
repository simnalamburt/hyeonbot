//! Personal IRC bot of Jihyeon Kim.
//!
//! The behavior mirrors the original Ruby implementation built on [Cinch]: the same handlers, the
//! same reply formatting (including Cinch's long message splitting) and the same reconnection
//! policy.
//!
//! [Cinch]: https://github.com/cinchrb/cinch

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;
use irc::client::prelude::*;
use tracing::{debug, error, info, warn};

pub mod db;
pub mod handlers;
pub mod split;

pub use db::Db;

const NICK: &str = "김젼봇";
const USER: &str = "hyeonbot";
const REALNAME: &str = "IRC bot. See https://github.com/simnalamburt/hyeonbot for the reference.";
/// Longest wait between reconnection attempts, in seconds.
const MAX_RECONNECT_DELAY: u64 = 60;

/// Runtime configuration of the bot.
#[derive(Clone, Debug)]
pub struct Settings {
    /// IRC server host
    pub server: String,
    /// IRC server port
    pub port: u16,
    /// Whether to connect over TLS
    pub use_tls: bool,
    /// Path of the SQLite database which remembers the joined channels
    pub db_path: PathBuf,
}

/// Errors that stop the bot.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// IRC connection failure
    #[error(transparent)]
    Irc(#[from] irc::error::Error),
    /// Database failure
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
}

/// Runs the bot forever, reconnecting whenever the connection drops.
///
/// Like Cinch, the delay between attempts doubles after each unsuccessful connection and is
/// capped at one minute; it resets after a connection that reached registration.
pub async fn run(settings: Settings) -> Result<std::convert::Infallible, Error> {
    let db = Arc::new(Db::open(&settings.db_path)?);
    let mut reconnects: u32 = 0;
    loop {
        let registered = match run_once(&settings, &db).await {
            Ok(registered) => registered,
            Err(Error::Irc(err)) => {
                error!("{}", describe(&err));
                false
            }
            Err(err) => return Err(err),
        };
        if registered {
            reconnects = 0;
        } else {
            reconnects += 1;
        }
        let wait = 2u64
            .checked_pow(reconnects)
            .unwrap_or(MAX_RECONNECT_DELAY)
            .min(MAX_RECONNECT_DELAY);
        info!("Waiting {wait} seconds before reconnecting");
        tokio::time::sleep(Duration::from_secs(wait)).await;
    }
}

/// Connects once and serves until the connection ends.
///
/// Returns whether the server completed registration (sent `RPL_WELCOME`) during this
/// connection, which drives the reconnection back-off.
pub async fn run_once(settings: &Settings, db: &Arc<Db>) -> Result<bool, Error> {
    let config = Config {
        nickname: Some(NICK.to_owned()),
        username: Some(USER.to_owned()),
        realname: Some(REALNAME.to_owned()),
        server: Some(settings.server.clone()),
        port: Some(settings.port),
        use_tls: Some(settings.use_tls),
        channels: db.channels()?,
        ..Default::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;
    let mut stream = client.stream()?;
    let sender = client.sender();
    let mut registered = false;

    while let Some(message) = stream.next().await.transpose()? {
        debug!(">> {}", message.to_string().trim_end());
        match &message.command {
            Command::Response(Response::RPL_WELCOME, _) => registered = true,

            Command::PRIVMSG(_, text) => {
                let Some(target) = message.response_target() else {
                    continue;
                };
                let target = target.to_owned();
                let text = text.clone();
                let nick = client.current_nickname().to_owned();
                let sender = sender.clone();
                // Dictionary lookups take a while; never block the read loop on them
                tokio::spawn(async move {
                    for reply in handlers::replies(&text).await {
                        if let Err(err) = send_reply(&sender, &nick, &target, &reply) {
                            error!("failed to reply to {target}: {err}");
                        }
                    }
                });
            }

            // React on invitation
            Command::INVITE(_, channel) => {
                sender.send_join(channel)?;
                if db.insert(channel)? {
                    info!("Hyeonbot has been invited to {channel}");
                } else {
                    warn!("Hyeonbot has been invited to the already joined channel.");
                }
            }

            Command::KICK(channel, kicked, _) if kicked == client.current_nickname() => {
                // Hyeonbot has been kicked, update DB
                db.delete(channel)?;
                info!("Hyeonbot has been kicked from {channel}");
            }

            _ => {}
        }
    }

    Ok(registered)
}

/// `err` followed by the chain of its causes, since `irc::error::Error` alone only says "an io
/// error occurred".
fn describe(err: &dyn std::error::Error) -> String {
    let mut description = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        description.push_str(": ");
        description.push_str(&cause.to_string());
        source = cause.source();
    }
    description
}

/// Sends `text` to `target` the way Cinch's `Message#reply` does: line by line, splitting lines
/// that would not fit in a single IRC message.
fn send_reply(sender: &Sender, nick: &str, target: &str, text: &str) -> irc::error::Result<()> {
    for line in text.lines() {
        for chunk in split::split_message(line, split::prefix_len(nick, USER, target)) {
            debug!("<< PRIVMSG {target} :{chunk}");
            sender.send_privmsg(target, chunk)?;
        }
    }
    Ok(())
}
