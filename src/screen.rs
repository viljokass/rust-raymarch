use super::vecmath;
use super::raymarch;

struct ScreenCoord {
    x: f64,
    y: f64,
}

pub struct Px {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Preparing for raymarch.
fn draw(screen_c: ScreenCoord) -> Px {

    let sx = screen_c.x;
    let sy = screen_c.y;

    let cam_depth = -5.0;

    // The starting point of the ray
    let ray_origin = vecmath::Vec3::from(0., 0., cam_depth);

    // The direction of the ray
    let screen = vecmath::Vec3::from(sx, sy, cam_depth + 1.);

    let ray_direction = screen.sub(&ray_origin).normalize();

    // Raymarching!
    raymarch::raymarch(&ray_origin, &ray_direction)
}

// Converts the pixels coming in into screen coordinates
fn px_to_screen(x: u32, y: u32, w: u32, h: u32, asp_rat: f64) -> ScreenCoord {
    // Convert the input parameters from u32 to f64
    let x: f64 = x as f64;
    let y: f64 = y as f64;
    let w: f64 = (w - 1) as f64;
    let h: f64 = (h - 1) as f64;
    // Construct the screen coordinates
    let sx = asp_rat * (2. * (x / w) - 1.);
    let sy = 2. * (y / h) - 1.;
    // Return the screen coordinate tuple
    ScreenCoord { x: sx, y: sy }
}

// A function that we use to raymarch.
// INPUT:
// * w: Width of the image
// * h: Height of the image
// * data: The vector of u8's in which the image is stored.
//         The image has to go in row-major order. For each pixel, one must
//         push 4 u8 values to the data vector: Red, Green, Blue and Alpha.
pub fn render(w: u32, h: u32, data: &mut Vec<u8>) {
    let aspect_ratio: f64 = (w as f64) / (h as f64);

    // We want to input the pixels from up-down, so we reverse the y-axis
    for j in (0..h).rev() {
        for i in 0..w {
            let sc = px_to_screen(i, j, w, h, aspect_ratio);
            let px: Px = draw(sc);
            data.push(px.r);
            data.push(px.g);
            data.push(px.b);
            data.push(px.a);
        }
    }
}
