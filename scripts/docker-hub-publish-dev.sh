#!/usr/bin/env bash

set -e

VERSION=$(git rev-parse --short HEAD)
NODE_NAME=eave/steam-node
BUILD_ARGS="--features with-steam-runtime --features=with-ethereum-compatibility"

docker build -f scripts/Dockerfile . -t $NODE_NAME:$VERSION --no-cache --build-arg GIT_COMMIT=${VERSION} --build-arg BUILD_ARGS="$BUILD_ARGS"
docker push $NODE_NAME:$VERSION