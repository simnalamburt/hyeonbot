Hyeonbot [![Docker Hub Status]][Docker Hub Link]
========
Personall IRC bot

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

<br>

How to run it
--------
```
# Using docker
docker run simnalamburt/hyeonbot

# If you want to use non-default config
#
# Environment Variables:
#   HYEONBOT_SERVER     Hostname of IRC server (default: "irc.ozinger.org")
#   HYEONBOT_PORT       Port number (default: "6697")
#   HYEONBOT_LOG_LEVEL  Log level, one of "debug", "log", "info", "warn", "error", "fatal" (default: "debug")
#
# NOTE: hyeonbot requires TLS connection
docker run \
  -e HYEONBOT_SERVER=irc.ozinger.org \
  -e HYEONBOT_PORT=6697 \
  simnalamburt/hyeonbot

# If you want persistency
docker run -d --restart=always \
  --mount type=bind,source=<YOUR CHOICE>,target=/a \
  simnalamburt/hyeonbot

# Without docker
gem install hyeonbot && hyeonbot
```

### How to build hyeonbot from source codes
```bash
gem build hyeonbot.gemspec
```

<br>

--------
*hyeonbot* is primarily distributed under the terms of both the [MIT license]
and the [Apache License (Version 2.0)]. See [COPYRIGHT] for details.


[Docker Hub Status]: https://badgen.net/docker/pulls/simnalamburt/hyeonbot/?icon=docker&label=pulls
[Docker Hub Link]: https://hub.docker.com/r/simnalamburt/hyeonbot/
[docker]: https://www.docker.com/
[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
