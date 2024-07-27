for f in *.jpg; do
    if [ ! -f ../out/"$f" ]; then
        convert "$f" -resize "1200x1200>" -dither FloydSteinberg -remap palette.alt.2.png -interpolate Nearest -filter point ../out/"$f"
    else
        echo "Skipping ${f}"
    fi
done