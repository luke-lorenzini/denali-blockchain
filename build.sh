#!/usr/bin/env bash

docker buildx build \
    --tag denali \
    --file Dockerfile \
    .
