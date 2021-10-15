#!/bin/bash
HOST=$1
if [ -z "$HOST" ];
    echo "Specify the host to get editor data from, e.g. foo@bar"
    exit
fi

OUTPUT=editor/data/data.$(date '+%Y%m%d').json
scp $HOST:/srv/half_earth_editor/data.json "$OUTPUT"
rsync -ravu $HOST:/srv/half_earth_editor/uploads/ editor/uploads

cp "$OUTPUT" editor/data.json
python3 parse_content.py
cd engine; cargo test; cd ..
npm run build-wasm