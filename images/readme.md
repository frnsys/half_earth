Serve images for both native and web.

For native the images are embedded into the binary and loaded directly. For web they're loaded via HTTP.

A note on performance: image loading/decoding on the web is slower because `egui` uses `image` to decode images rather than the built-in browser capacity, which is significantly faster. We could probably create a custom image loader for web that decodes the image using JS and passes the decoded bytes to `egui`. For now we just serve smaller JPEG images on web so that they load more quickly.
