#!/bin/bash
convert out/satellite.bw.full.png -interpolate Nearest -filter point -resize 50% /tmp/satellite.bw.png
pngquant /tmp/satellite.bw.png -o out/satellite.bw.png