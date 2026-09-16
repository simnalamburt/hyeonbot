// Copyright 2015-2026 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A port of Cinch's `Target#split_message`, which splits a reply that would exceed the 510-byte
//! IRC line limit into several messages joined with `...`.

/// Prepended to every continuation message (`message_split_start` of Cinch).
pub const SPLIT_START: &str = "... ";
/// Appended to every message that is continued (`message_split_end` of Cinch).
pub const SPLIT_END: &str = " ...";

/// The hostname part of our hostmask is unknown to the client, so the longest hostname allowed by
/// common IRC servers is budgeted for it.
const HOST_BUDGET: usize = 63;

/// Byte length of the prefix the server adds when it relays our `PRIVMSG` to `target`:
/// `:nick!user@host PRIVMSG target :`.
pub fn prefix_len(nick: &str, user: &str, target: &str) -> usize {
    ":".len()
        + nick.len()
        + "!".len()
        + user.len()
        + "@".len()
        + HOST_BUDGET
        + " PRIVMSG ".len()
        + target.len()
        + " :".len()
}

/// Splits `msg` so that every piece fits in `510 - prefix_len` bytes, breaking at whitespace when
/// possible and never inside a character.
pub fn split_message(msg: &str, prefix_len: usize) -> Vec<String> {
    let max = 510usize.saturating_sub(prefix_len);
    let max_without_end = max.saturating_sub(SPLIT_END.len());
    if msg.len() <= max {
        return vec![msg.to_owned()];
    }

    let mut pieces = Vec::new();
    let mut msg = msg.to_owned();
    while msg.len() > max_without_end {
        let chars: Vec<(usize, char)> = msg.char_indices().collect();

        // Index of the last character that still fits
        let mut acc = 0;
        let mut max_rune = 0;
        for (i, (_, ch)) in chars.iter().enumerate() {
            acc += ch.len_utf8();
            if acc > max_without_end {
                break;
            }
            max_rune = i;
        }

        // Prefer breaking at the last whitespace, but always take at least one character
        let r = chars[..=max_rune]
            .iter()
            .rposition(|(_, ch)| ch.is_ascii_whitespace())
            .unwrap_or(max_rune + 1)
            .max(1);
        let byte_r = chars.get(r).map_or(msg.len(), |(byte, _)| *byte);

        pieces.push(format!("{}{SPLIT_END}", &msg[..byte_r]));
        // Protect the spaces of SPLIT_START from the whitespace search of the next round
        msg = format!(
            "{}{}",
            SPLIT_START.replace(' ', "\x1a"),
            msg[byte_r..].trim_start()
        );
    }
    pieces.push(msg);
    pieces
        .into_iter()
        .map(|piece| piece.replace('\x1a', " "))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_message_is_untouched() {
        assert_eq!(split_message("hello", 100), ["hello"]);
    }

    #[test]
    fn splits_at_whitespace_and_marks_continuations() {
        // 510 - 480 = 30 bytes per message, 26 before " ..."
        let pieces = split_message("aaaa bbbb cccc dddd eeee ffff gggg hhhh iiii", 480);
        assert_eq!(
            pieces,
            ["aaaa bbbb cccc dddd eeee ...", "... ffff gggg hhhh iiii"]
        );
        for piece in &pieces {
            assert!(piece.len() <= 30, "{piece:?} is too long");
        }
    }

    #[test]
    fn never_splits_inside_a_character() {
        // 26 bytes before " ...": 8 three-byte characters in the first piece, then 7 after the
        // 4-byte "... " prefix, and the remainder in the last piece
        let pieces = split_message(&"가".repeat(40), 480);
        let counts: Vec<usize> = pieces
            .iter()
            .map(|piece| piece.chars().filter(|&c| c == '가').count())
            .collect();
        assert_eq!(counts, [8, 7, 7, 7, 7, 4]);
        for piece in &pieces[..pieces.len() - 1] {
            assert!(piece.ends_with(SPLIT_END));
            assert!(piece.len() <= 30, "{piece:?} is too long");
        }
        for piece in &pieces[1..] {
            assert!(piece.starts_with(SPLIT_START));
        }
    }

    #[test]
    fn continuation_prefix_is_not_a_break_point() {
        // The next round must not break right after "... "
        let pieces = split_message(&"x".repeat(60), 480);
        assert_eq!(pieces[0], format!("{}{SPLIT_END}", "x".repeat(26)));
        assert_eq!(
            pieces[1],
            format!("{SPLIT_START}{}{SPLIT_END}", "x".repeat(22))
        );
    }
}
