The scaling patterns are used to spatialize global temperature averages into a 320x160 grid.

1. Run the below scripts, to:
   1. Download the original scaling pattern data.
   2. Convert them into `.npz` files and generate an include file for Rust.
   3. (Optional) Preview the pattern outputs.

```shell
bash   01_download.sh
python 02_convert.py
# python 03_preview.py
```
