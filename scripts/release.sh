#!/usr/bin/env bash

set -euo pipefail

VERSION="${1:-}"

if [[ -z "$VERSION" ]]; then
    echo "usage: ./scripts/release.sh <version>"
    echo "example: ./scripts/release.sh 0.1.2"
    exit 1
fi

if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "error: version must be semver, e.g. 0.1.2"
    exit 1
fi

CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed -E 's/version = "([^"]+)"/\1/')

echo "Releasing hoshi $CURRENT_VERSION -> $VERSION"

if git diff --quiet; then
    :
else
    echo "error: working tree is dirty"
    git status --short
    exit 1
fi

if git rev-parse "$VERSION" >/dev/null 2>&1; then
    echo "error: tag $VERSION already exists"
    exit 1
fi

sed -i -E "0,/^version = \"[^\"]+\"/s//version = \"$VERSION\"/" Cargo.toml

cargo check

git add Cargo.toml Cargo.lock
git commit -m "release v$VERSION"
git push origin main

git tag "$VERSION"
git push origin "$VERSION"

echo
echo "bump hoshi to $VERSION"
echo "https://github.com/anuraglol/hoshi/releases/tag/$VERSION"
