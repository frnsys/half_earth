## @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
##
## This file documents the data objects included in the package.
##
## @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@


# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# The data frame Whittaker_biomes -----------------------------------------
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#' @title
#' Whittaker's biomes - data.frame
#'
#' @description
#' A data.frame containing boundaries of biomes as mean annual
#' temperature and precipitation. Digitized from **Figure 5.5** in
#' *Ricklefs, R. E. (2008). The economy of nature. W. H. Freeman and Company.*
#' (Chapter 5, Biological Communities, The biome concept).
#'
#' @format
#' A data frame with 775 rows and 4 variables:
#' \tabular{rlllllr}{
#'          \tab **Variable** \tab   \tab **Type**    \tab \tab **Description** \cr
#'   \[, 1] \tab **temp_c**   \tab , \tab *numeric*   \tab : \tab Mean annual temperature (degree Celsius)\cr
#'   \[, 2] \tab **precp_cm** \tab , \tab *numeric*   \tab : \tab Mean annual precipitation (cm)\cr
#'   \[, 3] \tab **biome_id** \tab , \tab *numeric*   \tab : \tab Biome's id\cr
#'   \[, 4] \tab **biome**    \tab , \tab *character* \tab : \tab Biome's name
#' }
#'
#' @details
#' Values in **temp_c** and **precp_cm** represent the vertices of the borders
#' between biome polygons as they were digitized.
#' For more details see [Whittaker_biomes_dataset](https://rawgit.com/valentinitnelav/plotbiomes/master/html/Whittaker_biomes_dataset.html)
#'
#' @examples
#' library(plotbiomes)
#' library(ggplot2)
#' ggplot() +
#'  geom_polygon(data = Whittaker_biomes,
#'               aes(x      = temp_c,
#'                   y      = precp_cm,
#'                   fill   = biome),
#'               colour = "gray98", # colour of polygon border
#'               size   = 0.5)      # thickness of polygon border
#'
#' # Run example in console with: example(Whittaker_biomes)
#' @md
"Whittaker_biomes"


# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# The spatial polygons Whittaker_biomes_poly ------------------------------
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#' @title
#' Whittaker's biomes - SpatialPolygonsDataFrame
#'
#' @description
#' Spatial polygons with coordinates as mean annual temperature and precipitation,
#' without a CRS, representing the boundaries of biomes
#' digitized from **Figure 5.5** in
#' *Ricklefs, R. E. (2008). The economy of nature. W. H. Freeman and Company.*
#' (Chapter 5, Biological Communities, The biome concept).
#'
#' @format
#' An object of class SpatialPolygonsDataFrame with 9 rows and 2 columns:
#' \tabular{rlllllr}{
#'          \tab **Variable** \tab   \tab **Type**    \tab \tab **Description** \cr
#'   \[, 1] \tab **biome_id** \tab , \tab *numeric*   \tab : \tab Biome's id\cr
#'   \[, 2] \tab **biome**    \tab , \tab *character* \tab : \tab Biome's name
#' }
#'
#' @seealso
#' \code{\link{Whittaker_biomes}} for the data.frame version
#' (which is required by \code{\link[ggplot2]{ggplot}}).
#' @md
"Whittaker_biomes_poly"


# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# The character vector Ricklefs_colors ------------------------------------
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#' @title
#' Biome colors from *Ricklefs (2008)*
#'
#' @description
#' Colors used for biome polygons in Figure 5.5 from *Ricklefs, R. E. (2008),
#' The economy of nature. W. H. Freeman and Company.*
#' (Chapter 5, Biological Communities, The biome concept).
#'
#' @format
#' Named character vector
#'
#' @examples
#' library(plotbiomes)
#' library(ggplot2)
#' ggplot() +
#'  geom_polygon(data = Whittaker_biomes,
#'               aes(x      = temp_c,
#'                   y      = precp_cm,
#'                   fill   = biome),
#'               colour = "gray98", # colour of polygon border
#'               size   = 0.5) +    # thickness of polygon border
#'  # fill the polygons with predefined colors
#'  scale_fill_manual(name   = "Whittaker biomes",
#'                    breaks = names(Ricklefs_colors),
#'                    labels = names(Ricklefs_colors),
#'                    values = Ricklefs_colors)
#'
#' # Run example in console with: example(Ricklefs_colors)
#' @md
"Ricklefs_colors"
