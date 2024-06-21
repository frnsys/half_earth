#!/bin/bash
convert "$1" -interpolate Nearest -filter point -resize 48 "out/$1"