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

### How to run it
The container image of *ircbot* project is uploaded to both [Quay] and [Docker
Hub].

Running with [rkt]:
```bash
rkt run --dns=host \
    --insecure-options=image \
    docker://quay.io/simnalamburt/ircbot
```
See [contrib/README.md](contrib/README.md) to

Running with [docker]:
```bash
docker run --detach \
    --name ircbot \
    --restart always \
    simnalamburt/ircbot
```

Running without container:
```bash
# Install dependencies
git clone https://github.com/simnalamburt/ircbot.git --depth=1 && cd ircbot
bundle

bundle exec ./run               # Debug mode
bundle exec ./run --production  # Production mode
```

<br>

--------
*ircbot* is primarily distributed under the terms of both the [MIT license]
and the [Apache License (Version 2.0)]. See [COPYRIGHT] for details.

[Docker Hub]: https://hub.docker.com/r/simnalamburt/ircbot/
[docker]: https://docker.com/
[Quay]: https://quay.io/repository/simnalamburt/ircbot
[rkt]: https://coreos.com/rkt
[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
