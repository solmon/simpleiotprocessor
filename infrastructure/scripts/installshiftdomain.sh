#!/bin/bash
cp /usr/local/domainshift.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable rtdomainshift.service
service rtdomainshift.service start

/usr/local/bin/webserver