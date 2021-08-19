#!/bin/bash
ffmpeg -i "$1" -vn -acodec libvorbis "${1%.mp3}.webm"