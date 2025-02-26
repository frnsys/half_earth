#!/bin/bash

dl_dir="data/src"
dl_sha="a0216e5e2e7493d089d183beeda5d2f9ddcbb592"
dl_url="https://github.com/JGCRI/hectorui/raw/$dl_sha/inst/shinyApp/www/maps"

mkdir -p "$dl_dir"

for file in \
  pr_Amon_CanESM2_rcp85_r1i1p1_200601-210012_pattern.rds        \
  pr_Amon_CESM1-BGC_rcp85_r1i1p1_200601-210012_pattern.rds      \
  pr_Amon_GFDL-ESM2G_rcp85_r1i1p1_200601-210012_pattern.rds     \
  pr_Amon_MIROC-ESM_rcp85_r1i1p1_200601-210012_pattern.rds      \
  pr_Amon_MPI-ESM-LR_rcp85_r1i1p1_200601-210012_pattern.rds     \
  pr_Amon_MRI-ESM1_rcp85_r1i1p1_200601-210012_pattern.rds       \
  tas_Amon_CanESM2_esmrcp85_r1i1p1_200601-210012_pattern.rds    \
  tas_Amon_CESM1-BGC_rcp85_r1i1p1_200601-210012_pattern.rds     \
  tas_Amon_GFDL-ESM2G_rcp85_r1i1p1_200601-210012_pattern.rds    \
  tas_Amon_MIROC-ESM_esmrcp85_r1i1p1_200601-210012_pattern.rds  \
  tas_Amon_MPI-ESM-LR_esmrcp85_r1i1p1_200601-210012_pattern.rds \
  tas_Amon_MRI-ESM1_esmrcp85_r1i1p1_200601-210012_pattern.rds
do
  curl --silent --output="$dl_dir"/"$file" "$dl_url"/"$file"
done
