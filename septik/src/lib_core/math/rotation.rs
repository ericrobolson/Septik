use super::*;

pub enum Axi {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rotation3d {
    /// The 'x' rotation
    pub pitch_radians: FixedNumber,
    /// The 'y' rotation
    pub yaw_radians: FixedNumber,
    /// The 'z' rotation
    pub roll_radians: FixedNumber,
}

impl Rotation3d {
    pub fn default() -> Self {
        Self {
            pitch_radians: 0.into(),
            yaw_radians: 0.into(),
            roll_radians: 0.into(),
        }
    }

    pub fn apply_to_vec3d(self, vec3d: Vec3d) -> Vec3d {
        let x_rot = Self::rotate_around_single_axis(self.pitch_radians, vec3d, Axi::X);
        let y_rot = Self::rotate_around_single_axis(self.yaw_radians, vec3d, Axi::Y);
        let z_rot = Self::rotate_around_single_axis(self.roll_radians, vec3d, Axi::Z);

        x_rot + y_rot + z_rot
    }

    pub fn rotate_vec3d_on_axis(self, vec3d: Vec3d, axis: Axi) -> Vec3d {
        match axis {
            Axi::X => Self::rotate_around_single_axis(self.pitch_radians, vec3d, axis),
            Axi::Y => Self::rotate_around_single_axis(self.yaw_radians, vec3d, axis),
            Axi::Z => Self::rotate_around_single_axis(self.roll_radians, vec3d, axis),
        }
    }

    fn rotate_around_single_axis(radians: FixedNumber, vec3d: Vec3d, axis: Axi) -> Vec3d {
        let x = vec3d.x;
        let y = vec3d.y;
        let z = vec3d.z;

        let rotated_vec = match axis {
            Axi::X => Vec3d::new(
                x,
                y * radians.cos() - z * radians.sin(),
                y * radians.sin() + z * radians.cos(),
            ),
            Axi::Y => Vec3d::new(
                x * radians.cos() + z * radians.sin(),
                y,
                -x * radians.sin() + z * radians.cos(),
            ),
            Axi::Z => Vec3d::new(
                x * radians.cos() - y * radians.sin(),
                x * radians.sin() + y * radians.cos(),
                z,
            ),
        };

        rotated_vec
    }
}

impl std::ops::Add for Rotation3d {
    type Output = Self;
    fn add(self, rhs: Rotation3d) -> <Self as std::ops::Add<Rotation3d>>::Output {
        return Self {
            pitch_radians: self.pitch_radians + rhs.pitch_radians,
            yaw_radians: self.yaw_radians + rhs.yaw_radians,
            roll_radians: self.roll_radians + rhs.roll_radians,
        };
    }
}

impl std::ops::AddAssign for Rotation3d {
    fn add_assign(&mut self, rhs: Rotation3d) {
        self.pitch_radians += rhs.pitch_radians;
        self.yaw_radians += rhs.yaw_radians;
        self.roll_radians += rhs.roll_radians;
    }
}

impl std::ops::Sub for Rotation3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> <Self as std::ops::Sub<Self>>::Output {
        return Self {
            pitch_radians: self.pitch_radians - rhs.pitch_radians,
            yaw_radians: self.yaw_radians - rhs.yaw_radians,
            roll_radians: self.roll_radians - rhs.roll_radians,
        };
    }
}

impl std::ops::SubAssign for Rotation3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.pitch_radians -= rhs.pitch_radians;
        self.yaw_radians -= rhs.yaw_radians;
        self.roll_radians -= rhs.roll_radians;
    }
}

impl std::ops::Mul for Rotation3d {
    type Output = Self;
    fn mul(self, rhs: Self) -> <Self as std::ops::Mul<Self>>::Output {
        return Self {
            pitch_radians: self.pitch_radians * rhs.pitch_radians,
            yaw_radians: self.yaw_radians * rhs.yaw_radians,
            roll_radians: self.roll_radians * rhs.roll_radians,
        };
    }
}

impl std::ops::MulAssign for Rotation3d {
    fn mul_assign(&mut self, rhs: Self) {
        self.pitch_radians *= rhs.pitch_radians;
        self.yaw_radians *= rhs.yaw_radians;
        self.roll_radians *= rhs.roll_radians;
    }
}

impl std::ops::Div for Rotation3d {
    type Output = Self;
    fn div(self, rhs: Self) -> <Self as std::ops::Div<Self>>::Output {
        return Self {
            pitch_radians: self.pitch_radians / rhs.pitch_radians,
            yaw_radians: self.yaw_radians / rhs.yaw_radians,
            roll_radians: self.roll_radians / rhs.roll_radians,
        };
    }
}

impl std::ops::DivAssign for Rotation3d {
    fn div_assign(&mut self, rhs: Self) {
        self.pitch_radians /= rhs.pitch_radians;
        self.yaw_radians /= rhs.yaw_radians;
        self.roll_radians /= rhs.roll_radians;
    }
}

impl Into<Vec3d> for Rotation3d {
    fn into(self) -> Vec3d {
        Vec3d::new(self.pitch_radians, self.yaw_radians, self.roll_radians)
    }
}
