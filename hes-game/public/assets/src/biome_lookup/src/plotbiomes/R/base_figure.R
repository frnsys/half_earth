#' Create the Whittaker biome base figure
#'
#' @description Creates the Whittaker biome figure from the vignette example.
#' This can be modified by passing additional \code{\link{ggplot2}} style
#' arguments to it.
#' @param color_palette A named or unnamed vector of length 9 that contains either
#' color names or values. If named, the names should correspond to biome names in the
#' \code{Whittaker_biomes} data object. See details for additional information.
#' The default is to use the colors from Figure 5.5 in Ricklefs, R. E. (2008),
#' \emph{The economy of nature} (Chapter 5, Biological Communities, The biome concept).
#' If the vector is not named, the function will insert the names automatically
#'
#' @return An object of class \code{gg} and \code{ggplot}.
#'
#' @details To specify your own color palette, create a named vector
#' where the names correspond to biome type and the values correspond to the
#' colors you'd like to use. This can either be numeric (e.g. 1,2,3, etc) or
#' character (e.g. 'red' or '#C1E1DD'). The names for each biome are as follows:
#' \itemize{
#'   \item{\code{Tundra}}
#'   \item{\code{Boreal forest}}
#'   \item{\code{Temperate seasonal forest}}
#'   \item{\code{Temperate rain forest}}
#'   \item{\code{Tropical rain forest}}
#'   \item{\code{Tropical seasonal forest/savanna}}
#'   \item{\code{Subtropical desert}}
#'   \item{\code{Temperate grassland/desert}}
#'   \item{\code{Temperate grassland/desert}}
#'   \item{\code{Woodland/shrubland}}
#' }
#'
#' If the vector is unnamed, the names from \emph{Ricklefs (2008)} will be
#' inserted automatically.
#'
#' Add additional features (e.g. \code{theme()} elements) using normal
#' \code{ggplot2} syntax. See examples.
#'
#' @examples
#'
#' library(ggplot2)
#' # Create the base plot
#'
#' whittaker_base_plot()
#'
#' # move the legend to top left corner, add border box,
#' # and adjust the background fill and grid.
#'
#' whittaker_base_plot() +
#'   theme(legend.position = c(0.15, 0.75),
#'         panel.background = element_blank(),
#'         panel.grid.major = element_line(gray(0.7)),
#'         panel.border = element_rect(fill = NA))
#'
#' @author Sam Levin, Valentin Stefan
#'
#' @import ggplot2
#' @importFrom utils data
#' @export

whittaker_base_plot <- function(color_palette = NULL) {
  utils::data('Whittaker_biomes', envir = environment())
  utils::data("Ricklefs_colors", envir = environment())

  # degree symbol debugging source:
  # https://stackoverflow.com/questions/37554118/ggplot-inserting-space-before-degree-symbol-on-axis-label
  xlabel <- expression("Temperature " ( degree*C))
  if(is.null(color_palette)) {
    color_palette <- Ricklefs_colors
  } else if(is.null(names(color_palette)) |
            any(is.na(names(color_palette)))) {
    # ^^second condition throws warning when names aren't specified.
    # consider changing

    names(color_palette) <- names(Ricklefs_colors)

    message("Names for 'color_palette' either not specified or too few",
            " were specified. Using names from 'Ricklefs_colors'.")
  }

  plt <- ggplot2::ggplot() +
    # add biome polygons
    ggplot2::geom_polygon(data = Whittaker_biomes,
                          ggplot2::aes(x    = temp_c,
                                       y    = precp_cm,
                                       fill = biome),
                          # adjust polygon border
                          colour = "gray98",
                          size   = 1) +
  # fill the polygons with predefined colors
    ggplot2::scale_fill_manual(name   = "Whittaker biomes",
                               breaks = names(color_palette),
                               labels = names(color_palette),
                               values = color_palette) +
    ggplot2::scale_x_continuous(xlabel) +
    ggplot2::scale_y_continuous('Precipitation (cm)')


  return(plt)
}
