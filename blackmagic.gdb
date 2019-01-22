target extended-remote /dev/ttyBmpGdb
monitor swdp_scan
attach 1
info mem
load
layout src
break main
run
