김젼봇
========
Personall IRC bot
```console
# Daum dictionary
<@사용자> dic elope
<@김젼봇> [ilóup]  눈이 맞아 함께 달아나다, 가출하다, 도망가다

<@사용자> d spado
<@김젼봇> [spéidou]  거세한 사람, 생식 불능자

# Count GitHub streak
<@사용자> 스트릭
<@김젼봇> 커밋 스트릭을 12일째 유지중, 오늘의 미션을 클리어하셨습니다. :3

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
