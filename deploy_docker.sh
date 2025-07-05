#!/bin/bash

ARCH=$(uname -m)
if [ -z "$ARCH" ]; then
  echo "Error: Unable to determine architecture"
  exit 1
fi

if [ "$ARCH" = "arm64" ]; then
  docker compose -f docker-compose-arm64.yml up
else
  docker compose -f docker-compose.yml up
fi
