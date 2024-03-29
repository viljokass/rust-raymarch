use crate::scene::{Scene, Material};
use crate::screen::Px;
use crate::vecmath::Vec3;

const MAX_DIST: f64 = 2500.0;
const MAX_ITER: u32 = 1000;
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
fn shade(p: &Vec3, n: &Vec3, cpos: &Vec3, scene: &Scene, mat: &Material) -> Option<Px> {
    let dir_to_cam = cpos.sub(p).normalize();

    // Matching done based on surface material
    match mat {
        // If the material is reflective, then raymarch again
        Material::Reflect => {
            let mut newscene = scene.clone();
            newscene.cpos = p.clone();
            let rd = dir_to_cam.scale(-1.).reflect(&n).normalize();
            let ro = &p.add(&rd.scale(2. * EPSILON));
            raymarch(&ro, &rd, &p, &newscene)
        },
        // If the material is color, then Phong shader.
        Material::Color { col } => {
            let lpos = &scene.lpos;
            let lcol = &scene.lcol;
            let acol = &scene.acol;

            let ambi_c: f64 = 0.1;
            let ambi_c = acol.scale(ambi_c);

            let dir_to_light = lpos.sub(p).normalize();

            let diff_c: f64 = f64::max(0.0, dir_to_light.dot(&n));
            let diff_c = &col.scale(diff_c).times(&lcol);

            let reflect = dir_to_light.scale(-1.).reflect(&n).normalize();
            let spec_c = f64::powi(f64::max(dir_to_cam.dot(&reflect), 0.0), 32);
            let spec_c = lcol.scale(spec_c);

            let sum_c = &spec_c.add(&diff_c).add(&ambi_c);

            Px::from(&vec![sum_c.x, sum_c.y, sum_c.z, 1.])
        }
    }
}

// Calculating normal using gradient approximation
fn calc_norm(p: &Vec3, scene: &Scene) -> Vec3 {
    Vec3 {
        x: scene.eval(&p.add(&DX)).sdfv - scene.eval(&p.sub(&DX)).sdfv,
        y: scene.eval(&p.add(&DY)).sdfv - scene.eval(&p.sub(&DY)).sdfv,
        z: scene.eval(&p.add(&DZ)).sdfv - scene.eval(&p.sub(&DZ)).sdfv,
    }
    .normalize()
}

// THE raymarch function.
pub fn raymarch(ro: &Vec3, rd: &Vec3, cpos: &Vec3, scene: &Scene) -> Option<Px> {
    let mut dist: f64 = 0.;
    for _i in 0..MAX_ITER {
        let pos: Vec3 = ro.add(&rd.scale(dist));
        let obj = scene.eval(&pos);
        let dist_to_nearest = obj.sdfv;
        if dist_to_nearest < EPSILON {
            // Surface normal
            let normal = calc_norm(&pos, &scene);
            return shade(&pos, &normal, &cpos, &scene, &obj.mat);
        }

        if dist > MAX_DIST {
            break;
        }

        dist += dist_to_nearest;
    }
    // If no hits, then return black.
    let ac = &scene.acol.scale(0.2);
    return Px::from(&vec![ac.x, ac.y, ac.z, 1.]);
}
