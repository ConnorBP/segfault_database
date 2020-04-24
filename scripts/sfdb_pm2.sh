#!/bin/sh
echo ""
echo "SegfaultDatabase pm2 Startup. Please ensure you have installed pm2/NodeJS/npm with the installer script first! Running SegfaultDatabase with pm2 means that pm2 runs SegfaultDatabase in the background of your machine and auto-restart even after reboot. If you are running the bot already, you can close the session you are currently using and start SegfaultDatabase with this method."

echo ""
echo ""
root=$(pwd)
    
echo ""
cd "$root"
echo "Starting sfdb in pm2 with auto-restart and no auto-update..."
sudo pm2 start "$root/sfdb_arn.sh" --interpreter=bash --name=SegfaultDatabase
sudo pm2 startup
sudo pm2 save
echo ""
echo "If you did everything correctly, pm2 should have started up sfdb! Please use sudo pm2 info SegfaultDatabase to check. You can view pm2 logs with sudo pm2 logs SegfaultDatabase"
