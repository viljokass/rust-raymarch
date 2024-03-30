use crate::vecmath::Vec3;

#[derive(Clone)]
pub enum Material {
    Reflect,
    Color {
        col: Vec3,
        shine: i32,
    },
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

// A function that makes scenes
pub fn create_scene() -> Scene {
    let mut objs: Vec<Obj> = Vec::new();

    let s1 = Obj {
        sdf: SDF::Sphere {
            pos: Vec3::from(-1.5, 0., -2.),
            rad: 1.,
        },
        mat: Material::Color {
            col: Vec3::from(1., 0.5, 0.2),
            shine: 32,
        },
    };
    objs.push(s1);

    let s2 = Obj {
        sdf: SDF::Sphere {
            pos: Vec3::from(1., 2., 0.),
            rad: 1.,
        },
        mat: Material::Reflect
    };
    objs.push(s2);

    let s3 = Obj {
        sdf: SDF::Sphere {
            pos: Vec3::from(2., -1., -1.),
            rad: 1.,
        },
        mat: Material::Color {
            col: Vec3::from(1., 0.6, 0.6),
            shine: 32,
        },
    };
    objs.push(s3);

    let p1 = Obj {
        sdf: SDF::Plane {
            pos: Vec3::from(0., -3., -3.),
            nor: Vec3::from(0., 2., -1.).normalize(),
            h: 0.01,
        },
        mat: Material::Color {
            col: Vec3::from(0.1, 0.1, 0.3),
            shine: 32,
        },
    };
    objs.push(p1);

    Scene {
        lpos: Vec3::from(0., 2., -3.),
        cpos: Vec3::from(0., 0., -5.),
        
        lcol: Vec3::from(1., 0.6, 0.4),
        acol: Vec3::from(0., 0., 0.),

        objs: objs,
    }
}
