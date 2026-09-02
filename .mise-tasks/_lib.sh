#!/usr/bin/env bash

info() { echo -e "  \e[36m->\e[0m $*"; }
step() { echo -e "\n\e[34m══ \e[1m$*\e[0m"; }
ok() { echo -e "  \e[32m\xe2\x9c\x93\e[0m $*"; }
err() { echo -e "  \e[31m\xe2\x9c\x97\e[0m $*"; }

init() {
    VERSION=$(grep -m 1 '^version = ' Cargo.toml | cut -d '"' -f 2)
    TAG="v$VERSION"
    ROOT="$(cd "$(dirname "$0")" && pwd)"
    while [ ! -f "$ROOT/Cargo.toml" ] && [ "$ROOT" != "/" ]; do
        ROOT="$(dirname "$ROOT")"
    done
    cd "$ROOT"
}

validate_version() {
    if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Usage: <version>"
        echo "  version must be semver (e.g., 0.8.0)"
        exit 1
    fi
}
