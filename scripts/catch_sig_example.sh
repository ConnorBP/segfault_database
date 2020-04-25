#!/bin/bash 

# https://unix.stackexchange.com/questions/146756/forward-sigterm-to-child-in-bash
# https://github.com/actix/actix-website/blob/master/content/docs/server.md
# https://rust-cli.github.io/book/in-depth/signals.html
# https://pm2.keymetrics.io/docs/usage/cluster-mode/

# Catch a SIGINT from pm2 and send a SIGTERM to actix for graceful shutdown

_term() { 
  echo "Caught SIGINT signal!" 
  // semd SIGTERM to the child proc instead
  kill -TERM "$child" 2>/dev/null
}

trap _term SIGINT

echo "Starting Server in background...";
# launch the server with trailing & so it goes into the background
./sfdb start &

child=$! 
wait "$child"