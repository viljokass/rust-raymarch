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

// SDF function form a sphere
fn sdf_sphere(p: &Vec3, r: f64) -> f64 {
   p.length() - r 
}

// A function in which we map the entire world.
fn map(p: &Vec3) -> f64 {
    let ball_pos = Vec3::from(0., 0., 0.,);
    sdf_sphere(&p.sub(&ball_pos), 1.)
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
pub fn raymarch(ro: &Vec3, rd: &Vec3) -> Px {
    let mut dist: f64 = 0.;
    for _i in 0..MAX_ITER {
        let pos: Vec3 = ro.add(&rd.scale(dist));
        let dist_to_nearest = map(&pos);
        if dist_to_nearest < EPSILON {
            let normal = calc_norm(&pos);
            return Px {
                r: 255,
                g: 127,
                b: 0,
                a: 255,
            };
        }

        if dist > MAX_DIST {
            break;
        }

        dist += dist_to_nearest;
    }
    return Px {
        r: 0,
        g: 127,
        b: 255,
        a: 255,
    };
}
