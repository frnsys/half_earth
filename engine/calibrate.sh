#!/bin/bash
# Usage:
# To re-compile, e.g. with a feature flag:
#   ./calibrate.sh build static_development
# To generate and analyze game trajectories under different scenarios:
#   ./calibrate.sh BanFossilFuels,Electrification,Nuclear

if [ "$1" == "build" ]; then
    FLAGS="$2" # e.g. "static_development,static_population,static_production"
    cargo build --release --example simulate --features "$FLAGS"
else
    SCENARIOS="$1" # Comma separated scenario names
    ./target/release/examples/simulate "$SCENARIOS"
    python plot.py
    firefox /tmp/plots/index.html
fi