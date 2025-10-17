#!/bin/bash

# Strip Japanese font to only have necessary glyphs.
pyftsubset NotoSansJP-Regular.ttf \
    --output-file NotoSansJP-Subset.ttf \
    --text-file=../../locales/jp.json \
    --layout-features='*' \
    --no-hinting \
    --desubroutinize \
    --ignore-missing-glyphs
