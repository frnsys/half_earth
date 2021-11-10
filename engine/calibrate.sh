#!/bin/bash
FLAGS=$1 # e.g. "static_development,static_population,static_production"
cargo run --example simulate --features "$FLAGS"
python plot.py
feh -FZd /tmp/plots/*.png