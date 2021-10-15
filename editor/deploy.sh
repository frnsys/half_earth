#!/bin/bash

npm run build

TARGET=ftseng@starbase:/srv/half_earth_editor
rsync -ravu ./ --copy-links --exclude=*.md --exclude=.git --exclude={node_modules,src,data.json,uploads} $TARGET