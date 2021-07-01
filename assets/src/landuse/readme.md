# Usage

1. Run `download.sh` to download the MODIS HDF files (listed in `files.txt`).
2. Run `process.sh` to mosaic the HDF files and project to EPSG:4326.
3. Run `python scale_and_remap.py` to scale and remap the labels and generate an output PNG for the frontend.

# Data

- [MCD12Q1 v006: MODIS/Terra+Aqua Land Cover Type Yearly L3 Global 500 m SIN Grid](https://lpdaac.usgs.gov/products/mcd12q1v006/)
    - Specifically: `MCD12Q1.006/2019.01.01`