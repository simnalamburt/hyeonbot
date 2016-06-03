So `systemd`, such production
========

### Install
```shell
sudo cp hyeonbot.service /etc/systemd/system/
sudo systemctl daemon-reload

sudo systemctl enable hyeonbot
sudo systemctl start hyeonbot
```

### Uninstall
```shell
sudo systemctl stop hyeonbot
sudo systemctl disable hyeonbot

sudo rm /etc/systemd/system/hyeonbot.service
```
