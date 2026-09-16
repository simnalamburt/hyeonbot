// Copyright 2015-2026 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Drives the bot against a minimal in-process IRC server.

use std::sync::Arc;
use std::time::Duration;

use hyeonbot::{Db, Settings};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

/// Waits until the bot sends a line starting with `expected`, returning it.
async fn expect_line<R: AsyncBufReadExt + Unpin>(reader: &mut R, expected: &str) -> String {
    let mut line = String::new();
    loop {
        line.clear();
        let n = tokio::time::timeout(Duration::from_secs(5), reader.read_line(&mut line))
            .await
            .expect("timed out waiting for the bot")
            .unwrap();
        assert!(n > 0, "bot disconnected while waiting for {expected:?}");
        let line = line.trim_end();
        if line.starts_with(expected) {
            return line.to_owned();
        }
    }
}

/// Waits for the next `PRIVMSG` the bot sends and returns its target and text. The irc crate
/// only prefixes the trailing parameter with `:` when it contains a space.
async fn expect_privmsg<R: AsyncBufReadExt + Unpin>(reader: &mut R) -> (String, String) {
    let line = expect_line(reader, "PRIVMSG ").await;
    let (target, text) = line["PRIVMSG ".len()..].split_once(' ').unwrap();
    (
        target.to_owned(),
        text.strip_prefix(':').unwrap_or(text).to_owned(),
    )
}

#[tokio::test]
async fn joins_remembered_channels_replies_and_tracks_invites_and_kicks() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("db");
    let db = Arc::new(Db::open(&db_path).unwrap());
    assert!(db.insert("#seed").unwrap());

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let settings = Settings {
        server: "127.0.0.1".to_owned(),
        port,
        use_tls: false,
        db_path: db_path.clone(),
    };

    let bot = tokio::spawn({
        let db = Arc::clone(&db);
        async move { hyeonbot::run_once(&settings, &db).await }
    });

    let (socket, _) = listener.accept().await.unwrap();
    let (read, mut write) = socket.into_split();
    let mut reader = BufReader::new(read);

    let nick = expect_line(&mut reader, "NICK ").await;
    assert_eq!(nick, "NICK 김젼봇");
    let user = expect_line(&mut reader, "USER ").await;
    assert!(user.starts_with("USER hyeonbot "), "{user}");
    write
        .write_all(b":mock 001 \xEA\xB9\x80\xEC\xA0\xBC\xEB\xB4\x87 :Welcome\r\n:mock 376 \xEA\xB9\x80\xEC\xA0\xBC\xEB\xB4\x87 :End of MOTD\r\n")
        .await
        .unwrap();

    // Channels from the database are joined on connect
    assert_eq!(expect_line(&mut reader, "JOIN ").await, "JOIN #seed");

    // Handlers reply to the channel, or to the user in a query
    write
        .write_all(":alice!a@h PRIVMSG #seed :ㅇㅅㅇ)b\r\n".as_bytes())
        .await
        .unwrap();
    assert_eq!(
        expect_privmsg(&mut reader).await,
        ("#seed".to_owned(), "d(ㅇㅅㅇ".to_owned())
    );
    write
        .write_all(":alice!a@h PRIVMSG 김젼봇 :우리나라 만세\r\n".as_bytes())
        .await
        .unwrap();
    assert_eq!(
        expect_privmsg(&mut reader).await,
        ("alice".to_owned(), "우리나라 → 한국".to_owned())
    );

    // Invitations are followed and remembered
    write
        .write_all(":alice!a@h INVITE 김젼봇 #new\r\n".as_bytes())
        .await
        .unwrap();
    assert_eq!(expect_line(&mut reader, "JOIN ").await, "JOIN #new");
    assert_eq!(db.channels().unwrap(), ["#seed", "#new"]);

    // Being kicked forgets the channel; someone else being kicked does not
    write
        .write_all(":op!o@h KICK #seed alice :bye\r\n:op!o@h KICK #new 김젼봇 :bye\r\n".as_bytes())
        .await
        .unwrap();
    // Round-trip through the bot so the kicks are surely processed
    write
        .write_all(":alice!a@h PRIVMSG #seed :>ㅅㅇ\r\n".as_bytes())
        .await
        .unwrap();
    assert_eq!(
        expect_privmsg(&mut reader).await,
        ("#seed".to_owned(), "ㅇㅅ<".to_owned())
    );
    assert_eq!(db.channels().unwrap(), ["#seed"]);

    // Closing the connection ends the session; registration was reached
    drop(write);
    drop(reader);
    let registered = tokio::time::timeout(Duration::from_secs(5), bot)
        .await
        .expect("bot did not stop after disconnect")
        .unwrap();
    assert!(matches!(registered, Ok(true)), "{registered:?}");
}

#[tokio::test]
async fn connection_refused_is_reported() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let dir = tempfile::tempdir().unwrap();
    let db = Arc::new(Db::open(&dir.path().join("db")).unwrap());
    let settings = Settings {
        server: "127.0.0.1".to_owned(),
        port,
        use_tls: false,
        db_path: dir.path().join("db"),
    };
    assert!(matches!(
        hyeonbot::run_once(&settings, &db).await,
        Err(hyeonbot::Error::Irc(_))
    ));
}
