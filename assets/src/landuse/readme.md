This generates a PNG of biome/landuse labels (using [IGBP Land Use Classification labels](https://lpdaac.usgs.gov/documents/101/MCD12_User_Guide_V6.pdf)) for the world.

The labels are:

- Evergreen Needleleaf Forests
- Evergreen Broadleaf Forests
- Deciduous Needleleaf Forests
- Deciduous Broadleaf Forests
- Mixed Forests
- Closed Shrublands
- Open Shrublands
- Woody Savannas
- Savannas
- Grasslands
- Permanent Wetlands
- Croplands
- Urban and Built-up Lands
- Cropland/Natural Vegetation Mosaics
- Permanent Snow and Ice
- Barren
- Water Bodies
- Unclassified

# Setup

1. Install [`pymodis`](http://www.pymodis.org/index.html), which provides `modis_mosaic.py` (used in `02_process.sh`)

# Usage

_Warning: This creates some large intermediary files (>10GB total)._

1. Run `01_download.sh` to download the MODIS HDF files into `src/hdf` (listed in `src/files.txt`). You will need to edit this file and add your <https://urs.earthdata.nasa.gov> login credentials.
2. Run `02_process.sh` to mosaic the HDF files and project to EPSG:4326.
3. Run `03_scale_and_remap.py` to scale and remap the labels and generate an output PNG map of landuse labels for the frontend.

# Data

- [MCD12Q1 v006: MODIS/Terra+Aqua Land Cover Type Yearly L3 Global 500 m SIN Grid](https://lpdaac.usgs.gov/products/mcd12q1v006/)
    - Specifically: `MCD12Q1.006/2019.01.01`
