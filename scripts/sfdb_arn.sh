#!/bin/sh
echo ""
echo "Running SegfaultDatabase with auto restart normally! (without updating)"
root=$(pwd)

sleep 5s
cd "$root"

while :; do cd "$root" && ./sfdb start; sleep 5s; done
echo ""
echo "Did something break? Uh oh I sure hope not."
sleep 3s

cd "$root"
#bash "$root/linuxAIO.sh"
echo "Done"

exit 0