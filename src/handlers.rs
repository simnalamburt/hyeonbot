//! Message handlers. Every handler whose pattern matches produces a reply, exactly like the
//! independent `on :message` handlers of the Ruby implementation.
//!
//! The optional `<nick> ` prefix in the patterns accepts messages relayed by discord-irc bridges.

use std::sync::LazyLock;

use regex::Regex;
use tracing::error;

/// Reply when the dictionary has nothing to say.
const NOT_FOUND: &str = "ㅇㅅㅇ)a";

static DIC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:<.*> )?[dD](?:ic)? (.+)$").unwrap());
static HIGHFIVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:<.*> )?ㅇㅅㅇ\)b$").unwrap());
static WINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:<.*> )?>ㅅㅇ$").unwrap());

/// Wraps `text` in Cinch's grey formatting (`Cinch::Formatting.format(:grey, text)`).
fn grey(text: &str) -> String {
    format!("\x0314{text}\x0f")
}

/// Replies of the synchronous handlers, in the order they are defined.
pub fn corrections(text: &str) -> Vec<String> {
    const KOREAN_GO_KR_120529: &str =
        "https://korean.go.kr/front/onlineQna/onlineQnaView.do?qna_seq=120529";
    let mut replies = Vec::new();

    // Health check
    if HIGHFIVE.is_match(text) {
        replies.push("d(ㅇㅅㅇ".to_owned());
    }
    if WINK.is_match(text) {
        replies.push("ㅇㅅ<".to_owned());
    }

    // 올바른 언어생활
    if text.contains("우리나라") {
        replies.push("우리나라 → 한국".to_owned());
    }
    if text.contains("한글화") {
        replies.push(format!(
            "한글화 → 한국어 번역 {}",
            grey("https://t.co/ztyockmyrj")
        ));
    }
    if text.contains("쉐이더") {
        replies.push(format!(
            "쉐이더 → 셰이더 {}",
            grey(&format!("쉘 → 셸, 쉐어 → 셰어 {KOREAN_GO_KR_120529}"))
        ));
    }
    if text.contains("쉐이크") {
        replies.push(format!(
            "쉐이크 → 셰이크 {}",
            grey(&format!("쉘 → 셸, 쉐어 → 셰어 {KOREAN_GO_KR_120529}"))
        ));
    }
    if text.contains("쉘") {
        replies.push(format!(
            "쉘 → 셸 {}",
            grey(&format!(
                "쉐이크 → 셰이크, 쉐어 → 셰어 {KOREAN_GO_KR_120529}"
            ))
        ));
    }
    if text.contains("쉐어") {
        replies.push(format!(
            "쉐어 → 셰어 {}",
            grey(&format!("쉐이크 → 셰이크, 쉘 → 셸 {KOREAN_GO_KR_120529}"))
        ));
    }
    if text.contains("됬") {
        replies.push(format!(
            "됬 → 됐 {}",
            grey("https://korean.go.kr/front/onlineQna/onlineQnaView.do?qna_seq=151151")
        ));
    }
    if text.contains("메세지") {
        replies.push(format!(
            "메세지 → 메시지 {}",
            grey("https://korean.go.kr/front/mcfaq/mcfaqView.do?mcfaq_seq=5548")
        ));
    }

    replies
}

/// The dictionary query in `text`, if it is a `dic`/`d` command.
pub fn dictionary_query(text: &str) -> Option<String> {
    DIC.captures(text)
        .map(|captures| captures[1].trim().to_lowercase())
}

/// Looks `query` up in the Daum dictionary. `None` means the lookup itself failed, in which case
/// nothing is replied (the Ruby handler raised and Cinch swallowed the exception).
async fn dictionary(query: &str) -> Option<String> {
    match daumdic::search(query).await {
        Ok(search) => {
            let result = search.to_string();
            Some(if result.is_empty() {
                NOT_FOUND.to_owned()
            } else {
                result
            })
        }
        Err(daumdic::DaumdicError::EmptyWord) => Some(NOT_FOUND.to_owned()),
        Err(err) => {
            error!("dictionary lookup of {query:?} failed: {err}");
            None
        }
    }
}

/// Every reply to `text`.
pub async fn replies(text: &str) -> Vec<String> {
    let mut replies = Vec::new();
    // Daum dictionary
    if let Some(query) = dictionary_query(text) {
        replies.extend(dictionary(&query).await);
    }
    replies.extend(corrections(text));
    replies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_command_forms() {
        assert_eq!(dictionary_query("d elope").as_deref(), Some("elope"));
        assert_eq!(dictionary_query("D elope").as_deref(), Some("elope"));
        assert_eq!(dictionary_query("dic  Elope ").as_deref(), Some("elope"));
        assert_eq!(dictionary_query("Dic 독수리").as_deref(), Some("독수리"));
        assert_eq!(
            dictionary_query("<alice> d elope").as_deref(),
            Some("elope")
        );
        assert_eq!(dictionary_query("dict elope"), None);
        assert_eq!(dictionary_query("d"), None);
        assert_eq!(dictionary_query("said elope"), None);
    }

    #[test]
    fn health_checks() {
        assert_eq!(corrections("ㅇㅅㅇ)b"), ["d(ㅇㅅㅇ"]);
        assert_eq!(corrections("<alice> ㅇㅅㅇ)b"), ["d(ㅇㅅㅇ"]);
        assert_eq!(corrections(">ㅅㅇ"), ["ㅇㅅ<"]);
        assert!(corrections("ㅇㅅㅇ)b!").is_empty());
    }

    #[test]
    fn language_corrections_stack_up() {
        assert_eq!(corrections("우리나라 최고"), ["우리나라 → 한국"]);
        assert_eq!(
            corrections("한글화 됬다"),
            [
                "한글화 → 한국어 번역 \x0314https://t.co/ztyockmyrj\x0f",
                "됬 → 됐 \x0314https://korean.go.kr/front/onlineQna/onlineQnaView.do?qna_seq=151151\x0f",
            ]
        );
        assert_eq!(
            corrections("쉘"),
            [
                "쉘 → 셸 \x0314쉐이크 → 셰이크, 쉐어 → 셰어 https://korean.go.kr/front/onlineQna/onlineQnaView.do?qna_seq=120529\x0f"
            ]
        );
        assert!(corrections("셸 스크립트").is_empty());
    }

    #[tokio::test]
    #[ignore = "requires access to the live Daum dictionary"]
    async fn dictionary_live() {
        let replies = replies("d Elope").await;
        assert_eq!(replies.len(), 1);
        assert!(replies[0].starts_with("[ilóup]  "), "{replies:?}");

        assert_eq!(replies_for("dic asdfaserqfasd").await, [NOT_FOUND]);
        assert_eq!(replies_for("d  ").await, [NOT_FOUND]);
    }

    async fn replies_for(text: &str) -> Vec<String> {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        replies(text).await
    }
}
