#!/usr/bin/env bash

if [ "$PORT" ]; then
  export APP_HTTP_SERVER__PORT="$PORT"
fi

./home/appuser/foruma-web
