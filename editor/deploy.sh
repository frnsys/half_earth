#!/bin/bash

ENV=$1

npm run build

TARGET=ftseng@starbase.in:/srv/half_earth_editor
rsync -ravu ./ --copy-links --exclude=*.md --exclude=.git --exclude={node_modules,src,data.json} $TARGET