#!/bin/sh
set -e
set -u

identifier="$1"
pid_file="/tmp/runonce/$identifier.pid"
shift 1

if [ -f "$pid_file" ]
then
	pid=$(cat "$pid_file") 
	if ps -p $pid > /dev/null
	then
		echo "process with identifier $identifier and pid $pid is still running"
		exit 0 
	fi
fi
mkdir -p $(dirname "$pid_file")
echo $$ > "$pid_file"

set +e
$@
rm "$pid_file"
