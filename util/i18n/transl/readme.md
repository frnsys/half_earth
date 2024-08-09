# Translations

Should be done in the `.csv` files with the first column being English and second column as the translation.

Note that empty translations should be indicated with `"-"`, as otherwise an empty string is considered as missing the translation.

Then use the `hes-game-i18n` binary to process them (via `just translate` from the project root).
