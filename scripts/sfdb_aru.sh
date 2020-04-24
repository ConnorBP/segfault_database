#!/bin/sh
echo ""
echo "Running SegfaultDatabase with auto restart and updating to latest build!"
root=$(pwd)

sleep 5s
while :; do cd "$root/segfault_database" && cargo clean && cargo build --release --bin sfdb && cd "$root/segfault_database/target/release" && cd "$root" && wget -N https://github.com/Kwoth/NadekoBot-BashScript/raw/1.9/nadeko_installer_latest.sh && bash "$root/nadeko_installer_latest.sh"; sleep 5s; done
echo ""
echo "That didn't work? Please report in #NadekoLog Discord Server."
sleep 3s

cd "$root"
echo ""
echo "Getting the pm2 startup options for NadekoBot.."
#wget -N https://github.com/Kaoticz/NadekoBot-BashScript/raw/1.9/nadekobotpm2start.sh && bash "$root/nadekobotpm2start.sh"
echo ""
sleep 2s
bash "$root/linuxAIO.sh"
echo "Done"

rm "$root/NadekoARU_Latest.sh"
exit 0