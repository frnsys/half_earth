#!/bin/bash
# Usage:
# To re-compile, e.g. with a feature flag:
#   ./calibrate.sh build static_development
# To generate and analyze game trajectories under different scenarios:
#   ./calibrate.sh BanFossilFuels,Electrification,Nuclear

# Exit if a command fails
set -e

if [ "$1" == "build" ]; then
    FLAGS="$2" # e.g. "static_development,static_population,static_production"
    cargo build --release --example simulate --features "$FLAGS"
else
    ./target/release/examples/simulate "$@"
    python plot.py
    firefox /tmp/plots/index.html
fi