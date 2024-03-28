use crate::raymarch;
use crate::scene::Scene;
use crate::vecmath::Vec3;

struct ScreenCoord {
    x: f64,
    y: f64,
}

pub enum Channel {
    Red,
    Green,
    Blue,
    Alpha,
}

pub struct Px {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Px {
    pub fn from(vals: &Vec<f64>) -> Option<Px> {
        let l = vals.len();
        if l != 4 {
            return None;
        }
        let px = Px {
            r: vals[0].clamp(0., 1.),
            g: vals[1].clamp(0., 1.),
            b: vals[2].clamp(0., 1.),
            a: vals[3].clamp(0., 1.),
        };
        Some(px)
    }

    pub fn get_ch(&self, channel: &Channel) -> f64 {
        match channel {
            Channel::Red => self.r,
            Channel::Green => self.g,
            Channel::Blue => self.b,
            Channel::Alpha => self.a,
        }
    }
}

// Preparing for raymarch.
fn draw(screen_c: ScreenCoord, scene: &Scene) -> Px {
    let sx = screen_c.x;
    let sy = screen_c.y;

    // The starting point of the ray
    let ray_origin = &scene.cpos;

    // The direction of the ray
    let screen = Vec3::from(sx, sy, &ray_origin.z + 1.);

    let ray_direction = screen.sub(&ray_origin).normalize();

    // Raymarching!
    raymarch::raymarch(&ray_origin, &ray_direction, &scene).unwrap()
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
pub fn render(w: u32, h: u32, data: &mut Vec<u8>, scene: &Scene) {
    let aspect_ratio: f64 = (w as f64) / (h as f64);

    // We want to input the pixels from up-down, so we reverse the y-axis
    for j in (0..h).rev() {
        for i in 0..w {
            let sc = px_to_screen(i, j, w, h, aspect_ratio);
            let px: Px = draw(sc, &scene);
            data.push((255. * px.get_ch(&Channel::Red)) as u8);
            data.push((255. * px.get_ch(&Channel::Green)) as u8);
            data.push((255. * px.get_ch(&Channel::Blue)) as u8);
            data.push((255. * px.get_ch(&Channel::Alpha)) as u8);
        }
    }
}
