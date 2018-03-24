#!/usr/bin/env bash

cargo build && \
  docker run --rm --name fetching -v ${PWD}/target/debug:/app/target/debug memes-tracker-fetching
