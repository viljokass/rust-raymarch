// A function that we use to raymarch.
// INPUT:
// * w: Width of the image
// * h: Height of the image
// * data: The vector of u8's in which the image is stored.
//         The image has to go in row-major order. For each pixel, one must
//         push 4 u8 values to the data vector: Red, Green, Blue and Alpha.
pub fn render(w: u32, h: u32, data: &mut Vec<u8>) {

    let delta_r = 1./(w - 1) as f64;
    let delta_g = 1./(h - 1) as f64;

    for i in 0..h {
        let g: u8 = (255. * (i as f64) * delta_g) as u8;
        for j in 0..w {
            let r: u8 = (255. * (j as f64) * delta_r) as u8;
            data.push(r);
            data.push(g);
            data.push(0);
            data.push(255);
        }
    }
}
