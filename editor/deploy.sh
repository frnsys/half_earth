#!/bin/bash

npm run build

TARGET=ftseng@starbase:/srv/projects/half_earth/editor
rsync -ravu ./ --copy-links --exclude=*.md --exclude=.git --exclude={node_modules,src,data.json,uploads,htpasswd} $TARGET