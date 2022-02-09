#!/bin/bash
for i in win/*; do
    convert $i -resize 800x800 -dither Floyd-Steinberg -remap ../palette.png -interpolate Nearest -filter point -resize 150% "../$i"
done
for i in lose/*; do
    convert $i -resize 800x800 -dither Floyd-Steinberg -remap ../palette.png -interpolate Nearest -filter point -resize 150% "../$i"
done