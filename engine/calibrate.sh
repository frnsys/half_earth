#!/bin/bash
cargo run --example simulate
python plot.py
feh -FZd /tmp/plots/*.png