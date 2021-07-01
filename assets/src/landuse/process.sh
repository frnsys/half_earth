#!/bin/bash

# Mosaic the HDF files (i.e. merge them into one)
# Use this to see the subdatasets and their numbers
#   gdalinfo MCD12Q1.A2019001.h22v13.006.2020212132220.hdf
# Here the IGBP land use classification should be subdataset 1
# <https://lpdaac.usgs.gov/documents/101/MCD12_User_Guide_V6.pdf>
ls | grep .hdf$ > hdf_files.txt
modis_mosaic.py -o mosaic.tif -s "1" hdf_files.txt

# Convert from MODIS's sinusoidal projection to EPSG:4326
gdalwarp -t_srs epsg:4326 A2019001_mosaic.tif ../mosaic.tif