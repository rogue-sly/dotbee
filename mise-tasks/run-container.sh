#!/bin/env bash

cargo build

docker run -it --rm -v $(pwd)/target/debug/dotsy:/usr/local/bin/dotsy:Z dotsy-dev
