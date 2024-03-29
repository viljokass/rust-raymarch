use crate::vecmath::Vec3;

#[derive(Clone)]
pub enum Material {
    Reflect,
    Color { col: Vec3 },
}

#[derive(Clone)]
pub enum SDF {
    Sphere { pos: Vec3, rad: f64},
    Plane { pos: Vec3, nor: Vec3, h: f64},
}

pub struct ObjRet {
    pub sdfv: f64,
    pub mat: Material,
}

#[derive(Clone)]
pub struct Obj {
    pub sdf: SDF,
    pub mat: Material,
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
#[derive(Clone)]
pub struct Scene {
    pub lpos: Vec3,
    pub cpos: Vec3,

    pub lcol: Vec3,
    pub acol: Vec3,

    pub objs: Vec<Obj>,
}

// Scene methods
impl Scene {
    pub fn eval(&self, p: &Vec3) -> ObjRet {
        let mut min = f64::MAX;
        let mut mat = &Material::Reflect;
        let objs = &self.objs;
        for v in objs {
            let cand = v.sdf.evaluate(&p);
            if cand < min {
                min = cand;
                mat = &v.mat;
            }
        }
        ObjRet {
            sdfv: min,
            mat: mat.clone(),
        }
    }
}
