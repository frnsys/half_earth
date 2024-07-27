#!/bin/bash
# Usage: ./build.sh <PLATFORM>, e.g. `./build.sh ITCH` or `./build.sh STEAM`
# Requires https://itchio.itch.io/butler

# exit when any command fails
set -e

if [ "$1" == "ITCH" ]; then
    ../itchio/butler push "../build/make/zip/linux/x64/Half-Earth Socialism-linux-x64-1.0.0.zip" frnsys/half-earth-socialism:linux-stable
    ../itchio/butler push "../build/make/zip/darwin/x64/Half-Earth Socialism-darwin-x64-1.0.0.zip" frnsys/half-earth-socialism:mac-stable
    ../itchio/butler push "../build/make/zip/win32/x64/Half-Earth Socialism-win32-x64-1.0.0.zip" frnsys/half-earth-socialism:win-stable
elif [ "$1" == "STEAM" ]; then
    cd ../steam
    ./deploy.sh
fi
