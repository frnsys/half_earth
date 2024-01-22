set dotenv-load

# Run the development server
run:
    npm start

# Run tests
test:
    npm run test

# Setup dev environment
setup:
    npm install -d
    cd hector/wasm && just all

# Update editor data
update:
    #!/usr/bin/env bash
    OUTPUT=editor/data/data.$(date '+%Y%m%d').json
    wget $EDITOR_URL -O $OUTPUT --user=$EDITOR_USER --password=$EDITOR_PASS
    cp $OUTPUT editor/data.json
    python3 parse_content.py
    # # cd engine; cargo test; cd ..
    npm run build-wasm

# Update using local editor data
update_local:
    python3 parse_content.py
    # # cd engine; cargo test; cd ..
    npm run build-wasm

# Build the Rust code
build_rust:
    npm run build-wasm
