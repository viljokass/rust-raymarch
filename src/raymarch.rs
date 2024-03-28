use super::vecmath::Vec3;
use super::screen::Px;

const MAX_DIST: f64 = 100.0;
const MAX_ITER: u32 = 100;
const EPSILON: f64 = 0.001;
const DX: Vec3 = Vec3 {
    x: EPSILON, 
    y: 0., 
    z: 0.,
};
const DY: Vec3 = Vec3 {
    x: 0.,
    y: EPSILON,
    z: 0.,
};
const DZ: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: EPSILON,
};
const LPOS: Vec3 = Vec3 {
    x: 0.,
    y: 0.3,
    z: -3.,
};

// SDF function form a sphere
fn sdf_sphere(p: &Vec3, r: f64) -> f64 {
   p.length() - r 
}

// A function in which we map the entire world.
// Currently has three spheres
fn map(p: &Vec3) -> f64 {

    let s1_pos = Vec3::from(1., 1., 0.,);
    let s1 = sdf_sphere(&p.sub(&s1_pos), 1.);

    let s2_pos = Vec3::from(-2., 0., -2.);
    let s2 = sdf_sphere(&p.sub(&s2_pos), 1.);

    let s3_pos = Vec3::from(0., -1., 3.);
    let s3 = sdf_sphere(&p.sub(&s3_pos), 1.);

    f64::min(f64::min(s1, s2), s3)
}

// Shading funtion - currently shades only diff light
// Object color is orange.
fn shade(p: &Vec3, n: &Vec3) -> Option<Px> {

    let dir_to_light = LPOS.sub(p).normalize();

    let diff_s: f64 = f64::max(0.0, dir_to_light.dot(&n));
    let diff_c: Vec3 = Vec3::from(1., 0.5, 0.).scale(diff_s);

    Px::from(&vec![diff_c.x, diff_c.y, diff_c.z, 1.])
}

// Calculating normal using gradient approximation
fn calc_norm(p: &Vec3) -> Vec3 {
    Vec3 {
        x: map(&p.add(&DX))-map(&p.sub(&DX)),
        y: map(&p.add(&DY))-map(&p.sub(&DY)),
        z: map(&p.add(&DZ))-map(&p.sub(&DZ)),
    }.normalize()
}

// THE raymarch function.
pub fn raymarch(ro: &Vec3, rd: &Vec3) -> Option<Px> {
    let mut dist: f64 = 0.;
    for _i in 0..MAX_ITER {
        let pos: Vec3 = ro.add(&rd.scale(dist));
        let dist_to_nearest = map(&pos);
        if dist_to_nearest < EPSILON {
            // Surface normal
            let normal = calc_norm(&pos);
            return shade(&pos, &normal);
        }

        if dist > MAX_DIST {
            break;
        }

        dist += dist_to_nearest;
    }
    // If no hits, then return black.
    return Px::from(&vec![0., 0., 0., 1.]);
}
