김젼봇
========

IRC 봇

```sh
# Install dependencies
bundle

./run               # Debug mode
./run --production  # Production mode
```

### Features

```console
# 사전기능
dic elope
[ilóup]  눈이 맞아 함께 달아나다, 가출하다, 도망가다

d spado
[spéidou]  거세한 사람, 생식 불능자

# 깃헙 스트릭
스트릭
커밋 스트릭을 12일째 유지중, 오늘의 미션을 클리어하셨습니다. :3

# 맞장구
ㅇㅅㅇ)b
d(ㅇㅅㅇ

>ㅅㅇ
ㅇㅅ<
```

<br>

Deploy with docker
--------
To prepare the docker image:

```shell
sudo docker build -t ircbot .
```

To run it:

```shell
sudo docker run --detach \
    --name ircbot \
    --restart always \
    ircbot
```

Etc:

```shell
# Attach to the running docker image
sudo docker exec -it ircbot /bin/sh

# Stop running docker container
sudo docker stop ircbot
```

<br>

--------
*ircbot* is primarily distributed under the terms of both the [MIT license]
and the [Apache License (Version 2.0)]. See [COPYRIGHT] for details.

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
