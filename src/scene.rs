use crate::vecmath::Vec3;

pub enum SDF {
    Sphere { pos: Vec3, rad: f64 },
    Plane { pos: Vec3, nor: Vec3, h: f64 },
}

impl SDF {
    pub fn evaluate(&self, p: &Vec3) -> f64 {
        match self {
            SDF::Sphere { pos, rad } => &p.sub(&pos).length() - rad,
            SDF::Plane { pos, nor, h } => &p.sub(pos).dot(&nor) + h,
        }
    }
}

// Scene struct
pub struct Scene {
    pub lpos: Vec3,
    pub cpos: Vec3,

    pub lcol: Vec3,
    pub acol: Vec3,

    pub objs: Vec<SDF>,
}

// Scene methods
impl Scene {
    pub fn eval(&self, p: &Vec3) -> f64 {
        let mut min = f64::MAX;
        let objs = &self.objs;
        for v in objs {
            let cand = v.evaluate(&p);
            if cand < min {
                min = cand;
            }
        }
        min
    }
}
