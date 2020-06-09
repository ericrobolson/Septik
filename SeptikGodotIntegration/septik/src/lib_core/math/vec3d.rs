use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3d {
    pub x: FixedNumber,
    pub y: FixedNumber,
    pub z: FixedNumber,
}

impl Vec3d {
    pub fn new(x: FixedNumber, y: FixedNumber, z: FixedNumber) -> Self {
        return Self { x: x, y: y, z: z };
    }

    pub fn default() -> Self {
        return Self::new(0.into(), 0.into(), 0.into());
    }

    pub fn multiply(&self, number: FixedNumber) -> Self {
        return Self {
            x: self.x * number,
            y: self.y * number,
            z: self.z * number,
        };
    }
}

impl std::ops::Neg for Vec3d {
    type Output = Self;
    fn neg(self) -> <Self as std::ops::Neg>::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add for Vec3d {
    type Output = Self;
    fn add(self, rhs: Vec3d) -> <Self as std::ops::Add<Vec3d>>::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl std::ops::AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Vec3d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> <Self as std::ops::Sub<Self>>::Output {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl std::ops::SubAssign for Vec3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul for Vec3d {
    type Output = Self;
    fn mul(self, rhs: Self) -> <Self as std::ops::Mul<Self>>::Output {
        return Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl std::ops::MulAssign for Vec3d {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::Div for Vec3d {
    type Output = Self;
    fn div(self, rhs: Self) -> <Self as std::ops::Div<Self>>::Output {
        return Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };
    }
}

impl std::ops::DivAssign for Vec3d {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Into<Vec3d> for (i32, i32, i32) {
    fn into(self) -> Vec3d {
        return Vec3d::new(self.0.into(), self.1.into(), self.2.into());
    }
}
