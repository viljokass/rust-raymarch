#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn from(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn add(&self, oth: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + oth.x,
            y: self.y + oth.y,
            z: self.z + oth.z,
        }
    }

    pub fn sub(&self, oth: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - oth.x,
            y: self.y - oth.y,
            z: self.z - oth.z,
        }
    }

    pub fn length(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        f64::sqrt(x * x + y * y + z * z)
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn dot(&self, oth: &Vec3) -> f64 {
        self.x * oth.x + self.y * oth.y + self.z * oth.z
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        let dp = self.dot(&normal);
        let minus = normal.scale(2.0 * dp);
        self.sub(&minus)
    }

    pub fn times(&self, oth: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * oth.x,
            y: self.y * oth.y,
            z: self.z * oth.z,
        }
    }
}
