# Rust Raymarch
A system that produces images using raymarching tehniques.
# Why
So that I learn me some Rust. Also I like raymarching.
# What

![the result.](https://github.com/viljokass/rust-raymarch/blob/main/img.png)

# About the structure
All starts at main, in which we ask image dimensions and create the scene.
We pass the dimensions, vector for storing pixels and the scene to the screen
module, which iterates over each pixel, converting pixels into screen coordinates.
During each iteration, we do raymarch - we also pass the scene to the raymarcher.

During the raymarch, we do usual raymarching stuff, evaluating the scene in order to
get the closest distance and gradient for the surface normal.

All this gets saved into a .png file. Cool stuff.
