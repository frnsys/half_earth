The scaling patterns are used to spatialize global temperature averages into a 320x160 grid.

1. Run `01_download.sh` to download the original scaling pattern data.
2. Run `02_convert.py` to convert the scaling patterns into `.npz` files and generate an include file for Rust.
3. (Optional) Run `03_preview.py` to preview the pattern outputs.