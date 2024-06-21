#' Identify temperature-precipitation outliers
#'
#' @description
#' Identifies temperature-precipitation pairs (points)
#' that are outside of the Whittaker biome polygons.
#'
#' @param tp
#'  A two column `matrix`, `data.frame` or `data.table` object.
#'  First column must refer to mean annual temperatures in Celsius degrees.
#'  Second column must refer to annual precipitations in cm.
#'  This two-column table represents all temperature-precipitation pairs (rows)
#'  to be tested if they fall inside or outside of the Whittaker biome polygons.
#'
#' @param validate
#'  Should the input be validated?
#'  Variable containing a single logical constant (`TRUE` or `FALSE`).
#'  Default is `TRUE`, meaning that some input checking is carried out by default.
#'  For example, it checks if `tp` variable contains exactly a two column
#'  `matrix`, `data.frame` or `data.table` object, if its columns are of
#'  `numeric` type and if the values are within certain ranges
#'  (e.g. temperature must be within -55 and 40 Celsius degrees
#'  and precipitation must not be above 1200 cm).
#'  See **Details** section for additional information.
#'
#' @details
#' Usually, for getting temperature-precipitation values one would most probably
#' have pairs of spatial coordinates. These coordinates would be used for raster
#' extraction (see `raster::extract()`).
#'
#' The ranges used for input validation are slightly wider that those encountered in the
#' ([CHELSA v 1.2](http://chelsa-climate.org/) BIO1 & BIO12 raster datasets.
#' The following can throw an error
#' (also possible reasons are given in the error messages):
#'
#' - Temperatures were switched with precipitations
#'   (temperature must be the first column);
#' - Temperature values were extracted from a raster that stores values as integers
#'   (for saving space, raster datasets could store data as integer type
#'   instead of full floating point accuracy; usually division by 10 solves the issue);
#' - Precipitations are in mm and not cm (divide by 10 to get cm);
#' - The given values are simply not temperature nor precipitation values.
#'   Perhaps other raster datasets were accidently used for point extraction.
#'
#' We consider useful and informative to include input validation.
#' There is always the option to turn the validation off by setting the argument `validate = FALSE`.
#'
#' @return
#' Returns a `data.table` with tree columns:
#' \tabular{rlllllr}{
#'          \tab **Column** \tab   \tab **Type**    \tab   \tab **Description** \cr
#'   \[, 1] \tab row_idx    \tab , \tab *integer*   \tab : \tab Outlier row index\cr
#'   \[, 2] \tab temp       \tab , \tab *numeric*   \tab : \tab Outlier mean annual temperature (Celsius degrees)\cr
#'   \[, 3] \tab pp_cm      \tab , \tab *numeric*   \tab : \tab Outlier annual precipitation (cm)
#' }
#' The indices in `row_idx` connect to the row indices in `tp` table.
#'
#' @author Valentin Stefan
#' @import data.table sp
#' @importFrom stats complete.cases
#' @export

get_outliers <- function(tp, validate = TRUE) {
  if (isTRUE(validate)) .validate_tp(tp)
  tp <- data.table::as.data.table(tp) # avoid setDT() - does not accept matrix class,
  # and also the object is set to DT in the global environment as well.
  # Set column names.
  data.table::setnames(x = tp, c("temp", "pp_cm"))
  # Keep track of row indices.
  tp[, row_idx := 1:.N]
  # Remove rows with missing values.
  tp_valid <- tp[complete.cases(tp)]
  # Make SpatialPoints using temperature-precipitation as coordinates.
  tp_valid_sp <- sp::SpatialPoints(coords = tp_valid[, -3])
  # Run spatial overlay between points and biome polygons.
  sp_overlay <- sp::over(x = tp_valid_sp,
                         y = as(Whittaker_biomes_poly, "SpatialPolygons"))
  # The spatial overlay function will return a vector of indices
  # where NA means that the point falls outside of biome polygons.
  # Therefore, NA-s can be used for identifying the outliers.
  outliers <- tp_valid[is.na(sp_overlay)]
  # Column reordering, so that row index is first column.
  data.table::setcolorder(x = outliers,
                          neworder = c("row_idx", "temp", "pp_cm"))
  return(outliers)
}


# Helper function ---------------------------------------------------------

# Helper function to validate temperature-precipitation input
.validate_tp <- function(tp) {
  # Test if 'tp' is of desired class
  is_cls_ok <- inherits(x = tp,
                        what = c("matrix", "data.frame", "data.table"))
  if (!is_cls_ok)
    stop(
      "\n
      'tp' is not a 'matrix', 'data.frame' or 'data.table'"
    )

  tp <- data.table::as.data.table(tp)
  # Test if 'tp' is a two column data.table
  if (dim(tp)[2] != 2)
    stop(
      "\n
      'tp' has more than two columns.
      Expecting a two column 'matrix', 'data.frame' or 'data.table':
      1 - first column must be mean annual temperatures in Celsius degrees
      2 - second column must be annual precipitations in cm"
    )

  # Check if the two columns are numeric.
  # The user should take care of the numeric conversion
  # because doing so can discover possible unwanted values.
  is_col_num <- sapply(tp, is.numeric)
  if (!all(is_col_num))
    stop(
      "\n
      Following column(s) in 'tp' argument is/are not numeric.
      Please convert to numeric: \n",
      paste(colnames(tp)[!is_col_num], collapse = "\n")
    )

  # Check if temperature and precipitation are within reasonable ranges:
  # - check temperatures
  if (!all(range(tp[, 1], na.rm = TRUE) %between% c(-55, 40)))
    stop(
      "\n
      Detected mean annual TEMPERATURE values below 55 C or above 40 C.
      The range of your data is ",
      paste(range(tp[, 1], na.rm = TRUE), collapse = " : "),
      "
      Check if temperature values need to be divided by 10.
      Otherwise check that first column represents temperatures in Celsius degrees
      and second column represents precipitations in cm (order matters)."
    )

  # - check precipitations
  if (!all(range(tp[, 2], na.rm = TRUE) %between% c(0, 1200)))
    stop(
      "\n
      Detected annual PRECIPITATION values below 0 or above 1200 cm.
      The range of your data is ",
      paste(range(tp[, 2], na.rm = TRUE), collapse = " : "),
      "
      Note that precipitation values must be in cm and not mm
      (convert from mm to cm dividing by 10).
      Otherwise check that first column represents temperatures in Celsius degrees
      and second column represents precipitations in cm (order matters)."
    )
}
