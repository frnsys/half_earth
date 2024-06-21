# test whittaker_base_plot()
context('Whittaker base plot')

library(ggplot2)

test_plot <- whittaker_base_plot()



test_that('function and arguments work correctly', {

  expect_true(inherits(test_plot, 'ggplot'))

  # colors are named properly when not user specified
  colors <- gray(0:9 / 9)

  expect_message(whittaker_base_plot(color_palette = colors),
                 regexp = "Names for 'color_palette'")

  tlc_legend <- whittaker_base_plot() +
    theme(legend.position = c(0.2, 0.75),
          panel.background = element_blank(),
          panel.grid.major = element_line(gray(0.2)),
          panel.border = element_rect(fill = NA))

  expect_silent(tlc_legend)

})


