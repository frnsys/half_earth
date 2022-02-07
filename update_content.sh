#!/bin/bash
# Exit if any command fails
set -e

OUTPUT=editor/data/data.$(date '+%Y%m%d').json
wget "http://half-earth-editor.frnsys.com/data" -O "$OUTPUT"

cp "$OUTPUT" editor/data.json
python3 parse_content.py
# cd engine; cargo test; cd ..
npm run build-wasm