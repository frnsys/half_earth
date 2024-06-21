# Declare global variables and native symbol objects ----------------------

# Doing so, avoids the note from devtools::check():
# "no visible binding for global variable".
# See https://stackoverflow.com/a/12429344/5193830
# or https://stackoverflow.com/a/17807914/5193830

.onLoad <- function(...) {
  if (getRversion() >= "2.15.1")
    utils::globalVariables(
      c(
        'biome',
        'biome_id',
        'Whittaker_biomes',
        'Whittaker_biomes_poly',
        'Ricklefs_colors',
        'row_idx',
        'temp_c',
        'precp_cm',
        # from map_outliers():
        'x',
        'y',
        # data.table's '.'
        '.',
        # methods::as used in get_outliers()
        'as'
      )
    )
}


# Package startup message -------------------------------------------------

.onAttach <- function(...) {
  packageStartupMessage(strwrap("Happy biome plotting!", indent = 5))
}
