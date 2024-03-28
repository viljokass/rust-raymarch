use crate::vecmath::Vec3;
use crate::screen::Px;
use crate::scene::Scene;

const MAX_DIST: f64 = 500.0;
const MAX_ITER: u32 = 250;
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

// Shading funtion - currently shades only diff light
// Object color is orange.
fn shade(p: &Vec3, n: &Vec3, scene: &Scene) -> Option<Px> {

    let lpos = &scene.lpos;

    let dir_to_light = lpos.sub(p).normalize();

    let diff_s: f64 = f64::max(0.0, dir_to_light.dot(&n));
    let diff_c: Vec3 = Vec3::from(1., 0.5, 0.).scale(diff_s);

    Px::from(&vec![diff_c.x, diff_c.y, diff_c.z, 1.])
}

// Calculating normal using gradient approximation
fn calc_norm(p: &Vec3, scene: &Scene) -> Vec3 {
    Vec3 {
        x: scene.eval(&p.add(&DX))-scene.eval(&p.sub(&DX)),
        y: scene.eval(&p.add(&DY))-scene.eval(&p.sub(&DY)),
        z: scene.eval(&p.add(&DZ))-scene.eval(&p.sub(&DZ)),
    }.normalize()
}

// THE raymarch function.
pub fn raymarch(ro: &Vec3, rd: &Vec3, scene: &Scene) -> Option<Px> {
    let mut dist: f64 = 0.;
    for _i in 0..MAX_ITER {
        let pos: Vec3 = ro.add(&rd.scale(dist));
        let dist_to_nearest = scene.eval(&pos);
        if dist_to_nearest < EPSILON {
            // Surface normal
            let normal = calc_norm(&pos, &scene);
            return shade(&pos, &normal, &scene);
        }

        if dist > MAX_DIST {
            break;
        }

        dist += dist_to_nearest;
    }
    // If no hits, then return black.
    return Px::from(&vec![0., 0., 0., 1.]);
}
