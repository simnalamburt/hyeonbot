Hyeonbot
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
#### A. With [docker]
```bash
docker run -d --restart=always \
  -e HYEONBOT_SERVER=irc.ozinger.org \
  -e HYEONBOT_PORT=6697 \
  simnalamburt/hyeonbot
```

#### B. Without container
```bash
# Install dependencies
bundle

# Update config
vim config.yaml

bundle exec run
```

<br>

--------
*hyeonbot* is primarily distributed under the terms of both the [MIT license]
and the [Apache License (Version 2.0)]. See [COPYRIGHT] for details.

[docker]: https://docker.com/
[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
