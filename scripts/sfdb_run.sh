#!/bin/sh
echo ""
echo "SFDB Launcher 1.0"
echo "segfault was here."
root=$(pwd)

cd "$root"
./sfdb start
echo "Done"

cd "$root"
exit 0