for f in *.jpg; do
    convert "$f" -resize 50% -dither FloydSteinberg -remap palette.alt.2.png -interpolate Nearest -filter point -resize 150% ../out/"$f"
done