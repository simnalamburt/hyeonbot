So `systemd`, such production
========

### Install
```shell
sudo cp ircbot.service /etc/systemd/system/
sudo systemctl daemon-reload

sudo systemctl enable ircbot
sudo systemctl start ircbot
```

### Uninstall
```shell
sudo systemctl stop ircbot
sudo systemctl disable ircbot

sudo rm /etc/systemd/system/ircbot.service
```
