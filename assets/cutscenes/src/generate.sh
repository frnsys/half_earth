for f in *.jpg; do
    if [ ! -f ../out/"$f" ]; then
        convert "$f" -resize "800x800>" -dither FloydSteinberg -remap palette.alt.2.png -interpolate Nearest -filter point -resize 150% ../out/"$f"
    else
        echo "Skipping ${f}"
    fi
done