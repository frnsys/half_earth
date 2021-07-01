#!/bin/bash

# Use urs.earthdata.nasa.gov login credentials
# More info: <https://lpdaac.usgs.gov/products/mcd12q1v006/>
USER=""
PASSWORD=""

# Download individual HDF files
# files.txt is generated from:
# <https://e4ftl01.cr.usgs.gov/MOTA/MCD12Q1.006/2019.01.01/>
mkdir hdf; cd hdf
wget --user=$USER --password=$PASSWORD -i ../files.txt