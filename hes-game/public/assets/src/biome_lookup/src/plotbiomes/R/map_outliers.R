#' View Whittaker outliers on an interactive map
#'
#' @description
#' This function makes use of the `mapview` package functionality to produce
#' interactive views of temperature-precipitation Whittaker outliers.
#'
#' @param tp
#'  A two column `matrix`, `data.frame` or `data.table` object.
#'  First column must refer to mean annual temperatures in Celsius degrees.
#'  Second column must refer to annual precipitations in cm.
#'  This two-column table represents all temperature-precipitation pairs (rows)
#'  to be tested if they fall inside or outside of the Whittaker biome polygons.
#'
#' @param xy
#'  A two column `matrix`, `data.frame` or `data.table` object containing
#'  the spatial coordinates corresponding to the temperature-precipitation
#'  pairs (rows) from the `tp` table above.
#'  First column must be WGS84 longitude/easting.
#'  Second column must be WGS84 latitude/northing.
#'
#' @param validate
#'  Should the input be validated?
#'  Variable containing a single logical constant (`TRUE` or `FALSE`).
#'  Default is `TRUE`, meaning that some input checking is carried out by default.
#'  Passed to `get_outliers`. See `help(get_outliers)` for details.
#'  Checks also if `xy` is a two column `matrix`, `data.frame` or `data.table`,
#'  if its columns are of `numeric` type and if `xy` and `tp` tables
#'  have the same number of rows.
#'
#' @param basemaps
#'  Passed to `mapview::mapviewOptions`.
#'  Default values are:
#'  basemaps = c("Esri.WorldImagery",
#'               "OpenTopoMap",
#'               "Esri.WorldShadedRelief",
#'               "OpenStreetMap.Mapnik",
#'               "CartoDB.Positron")
#'
#' @param ...
#'  Other arguments passed to `mapview::mapView()`.
#'
#' @return
#' Interactive view (map) of Whittaker outliers
#' as produced by `mapview::mapView()` function.
#'
#' @author Valentin Stefan
#' @import data.table sp mapview
#' @export


map_outliers <- function(tp,
                         xy,
                         validate = TRUE,
                         basemaps = c(
                           "Esri.WorldImagery",
                           "OpenTopoMap",
                           "Esri.WorldShadedRelief",
                           "OpenStreetMap.Mapnik",
                           "CartoDB.Positron"),
                         ...) {
  if (isTRUE(validate)) .validate_xy(xy, tp)
  xy <- data.table::as.data.table(xy) # avoid setDT() - does not accept matrix class,
  # and also the object is set to DT in the global environment as well.
  # Get biome outliers and their longitude-latitude coordinates.
  outliers <- get_outliers(tp, validate)
  outliers[, c("x", "y") := xy[row_idx]]
  # Make SpatialPointsDataFrame
  outliers_WGS84 <-
    sp::SpatialPointsDataFrame(
      coords      = outliers[, .(x, y)],
      data        = outliers,
      proj4string = CRS("+proj=longlat +datum=WGS84 +ellps=WGS84 +towgs84=0,0,0")
    )
  # Set basemaps.
  mapview::mapviewOptions(basemaps = basemaps)
  # Make interactive map.
  mapview::mapView(outliers_WGS84, ...)
}


# Helper function ---------------------------------------------------------

# Helper function to validate the 'xy' coordinates
.validate_xy <- function(xy, tp) {
  # Test if 'xy' is of desired class
  is_cls_ok <- inherits(x = xy,
                        what = c("matrix", "data.frame", "data.table"))
  if (!is_cls_ok)
    stop("\n
         'xy' is not a 'matrix', 'data.frame' or 'data.table'")

  xy <- data.table::as.data.table(xy)
  # Test if 'xy' is a 2 column data.table
  if (dim(xy)[2] != 2)
    stop(
      "\n
      'xy' has more than two columns.
      Expecting a 2 column 'matrix', 'data.frame' or 'data.table':
      1 - first column must be WGS84 longitude/easting
      2 - second column must be WGS84 latitude/northing"
    )

  # Check if the two columns are numeric.
  # The user should take care of the numeric conversion
  # because doing so can discover possible unwanted values.
  is_col_num <- sapply(xy, is.numeric)
  if (!all(is_col_num))
    stop(
      "\n
      Following column(s) in 'xy' argument are not numeric.
      Please convert to numeric: \n",
      paste(colnames(xy)[!is_col_num], collapse = "\n")
    )

  # Check longitude & latitude
  is_long_ok <- all(range(xy[, 1], na.rm = TRUE) %between% c(-180, 180))
  if (!is_long_ok)
    stop(
      "\n
      Longitude is not between -180 and 180 degrees (WGS84).
      Make sure also that longitude was not switched with latitude."
    )
  is_lat_ok <- all(range(xy[, 2], na.rm = TRUE) %between% c(-90, 90))
  if (!is_long_ok)
    stop(
      "\n
      Latitude is not between -90 and 90 degrees (WGS84).
      Make sure also that latitude was not switched with longitude."
    )

  # check also that xy and tp have same number of rows
  if (dim(xy)[1] != dim(tp)[1])
    stop(
      "\n
      'xy' and 'tp' have different number of rows.
      'xy' has ",
      paste(dim(xy)[1]),
      " and 'tp' has ",
      paste(dim(tp)[1]),
      "
      The temperature-precipitation extracted values do not
      correspond with the given spatial coordinates."
    )
}
