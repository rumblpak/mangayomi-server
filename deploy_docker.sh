#!/bin/bash

ARCH=$(uname -m)
if [ -z "$ARCH" ]; then
  echo "Error: Unable to determine architecture"
  exit 1
fi

if [ "$ARCH" = "arm64" ]; then
  docker compose up -f docker-compose-arm64.yml
else
  docker compose up -f docker-compose.yml
fi
