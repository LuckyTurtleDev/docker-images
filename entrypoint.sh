#!/bin/sh
set -e

user=dockeruser

chown "$user" -R .

crontab -u "$user"  /tasks.cron && crond -f -L /dev/stdout
