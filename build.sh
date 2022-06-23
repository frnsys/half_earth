#!/bin/bash
# Build Electron apps for itch.io
# Requires https://itchio.itch.io/butler

VERSION=1.0.0

# exit when any command fails
set -e

MAINDIR=$(pwd)
function realizeSymlinks() {
    cd "$1/app/assets"
    for f in $(find . -type l); do
        SOURCE="$MAINDIR/assets/$(dirname "$f")/$(readlink "$f")"
        echo "Copying $SOURCE to $f"
        rm -r "$f"
        cp -r --remove-destination "$SOURCE" "$f"
    done
    cd "$MAINDIR"
    echo "Removing dirs"
    rm -rf "$1/app/sharing"
    rm -rf "$1/app/assets/src"
    rm -rf "$1/app/assets/cutscenes/src"
    rm -rf "$1/app/assets/environments/src"
    rm -rf "$1/app/assets/backgrounds/src"
}

rm -rf out

npm run build-wasm
npm run build

# win32 requires a clean wine prefix,
# since mine main prefix is configured in a way that
# is incompatible with electron-forge.
npx electron-forge package --platform linux
npx electron-forge package --platform darwin
WINEPREFIX=~/work/half_earth/wine npx electron-forge package --platform win32

# Have to manually replace symlinks,
# Electron can't seem to do it on its own??
realizeSymlinks "out/Half-Earth Socialism-darwin-x64/Half-Earth Socialism.app/Contents/Resources"
realizeSymlinks "out/Half-Earth Socialism-linux-x64/resources"
realizeSymlinks "out/Half-Earth Socialism-win32-x64/resources"

npx electron-forge make --skip-package --platform linux
npx electron-forge make --skip-package --platform darwin
npx electron-forge make --skip-package --platform win32

./platforms/itchio/butler push "./out/make/zip/linux/x64/Half-Earth Socialism-linux-x64-${VERSION}.zip" frnsys/half-earth-socialism:linux-stable
./platforms/itchio/butler push "./out/make/zip/darwin/x64/Half-Earth Socialism-darwin-x64-${VERSION}.zip" frnsys/half-earth-socialism:mac-stable
./platforms/itchio/butler push "./out/make/zip/win32/x64/Half-Earth Socialism-win32-x64-${VERSION}.zip" frnsys/half-earth-socialism:win-stable