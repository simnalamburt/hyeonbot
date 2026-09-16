Hyeonbot
========
Personal IRC bot

```console
# Daum dictionary
<@사용자> dic elope
<@김젼봇> [ilóup]  눈이 맞아 함께 달아나다, 가출하다, 도망가다

<@사용자> d spado
<@김젼봇> [spéidou]  거세한 사람, 생식 불능자

# Fix mistakes
<@사용자> 한글화
<@김젼봇> 한글화 → 한국어 번역 https://t.co/ztyockmyrj 참고

# Highfive
<@사용자> ㅇㅅㅇ)b
<@김젼봇> d(ㅇㅅㅇ

<@사용자> >ㅅㅇ
<@김젼봇> ㅇㅅ<
```

&nbsp;

### How to run
```bash
cargo install --git https://github.com/simnalamburt/hyeonbot && hyeonbot
```

Hyeonbot connects to `irc.ozinger.org:6697` over TLS and remembers the channels
it has been invited to in a SQLite database named `db` in the working
directory. Configure it with environment variables:

| Variable             | Default            |
|----------------------|--------------------|
| `HYEONBOT_SERVER`    | `irc.ozinger.org`  |
| `HYEONBOT_PORT`      | `6697`             |
| `HYEONBOT_LOG_LEVEL` | `debug`            |

Build hyeonbot from source codes:
```bash
cargo build --release
```

&nbsp;

--------
*hyeonbot* is primarily distributed under the terms of both the [MIT license]
and the [Apache License (Version 2.0)]. See [COPYRIGHT] for details.

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
