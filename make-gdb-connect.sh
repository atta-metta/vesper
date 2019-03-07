#!/bin/sh
# Generate gdb-connect script with given RTT block address, to avoid typing it in manually

# Hint from this answer https://superuser.com/a/747905/355774
[ $# -ge 1 ] && ADDR="$1" || ADDR=$(cat)

cat <<EOF > gdb-connect
target remote :3333
monitor rttserver start 19021 0
monitor rtt setup 0x$ADDR 24 "SEGGER RTT"
EOF