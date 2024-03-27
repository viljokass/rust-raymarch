struct Px {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// Maybe I should name these better functions better, since this isn't ray marching yet.
fn raymarch(screen_c: (f64, f64)) -> Px {
    let x = screen_c.0;
    let y = screen_c.1;
    if (x * x + y * y) < 1. {
        return Px {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
    }
    Px {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }
}

// Converts the pixels coming in into screen coordinates
fn px_to_screen(x: u32, y: u32, w: u32, h: u32, asp_rat: f64) -> (f64, f64) {
    // Convert the input parameters from u32 to f64
    let x: f64 = x as f64;
    let y: f64 = y as f64;
    let w: f64 = (w - 1) as f64;
    let h: f64 = (h - 1) as f64;
    // Construct the screen coordinates
    let sx = asp_rat * (2. * (x / w) - 1.);
    let sy = 2. * (y / h) - 1.;
    // Return the screen coordinate tuple
    (sx, sy)
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
            let c = px_to_screen(i, j, w, h, aspect_ratio);
            let px: Px = raymarch(c);
            data.push(px.r);
            data.push(px.g);
            data.push(px.b);
            data.push(px.a);
        }
    }
}
