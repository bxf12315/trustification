#!/usr/bin/env bash

set -e

echo "Trunk profile: $TRUNK_PROFILE"
echo "Production profile: $PRODUCTION"
if [[ "$TRUNK_PROFILE" == "debug" ]]; then
  if [[ "$PRODUCTION" == "DEV" ]]; then
    if [ -n "$(netstat -an | grep 8025)" ]; then
        echo "Port 8023 is in use"
      else
        OLD_PWD=$PWD
        cd mock-server
        cargo run &
        cd $OLD_PWD
      fi
  fi
fi