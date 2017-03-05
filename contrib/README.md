Run 김젼봇 as a systemd service
--------

### A. Install
```shell
sudo cp ircbot.service /etc/systemd/system/
sudo systemctl daemon-reload

sudo systemctl enable ircbot
sudo systemctl start ircbot
```

### B. Uninstall
```shell
sudo systemctl stop ircbot
sudo systemctl disable ircbot

sudo rm /etc/systemd/system/ircbot.service
```

###### References
- https://coreos.com/rkt/docs/latest/using-rkt-with-systemd.html
