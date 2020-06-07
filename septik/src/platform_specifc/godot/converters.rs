use crate::lib_core;
use lib_core::math::*;

impl Into<gdnative::Vector2> for Vec3d {
    fn into(self) -> gdnative::Vector2 {
        return gdnative::Vector2::new(self.x.into(), self.y.into());
    }
}

impl Into<gdnative::Vector3> for Vec3d {
    fn into(self) -> gdnative::Vector3 {
        return gdnative::Vector3::new(self.x.into(), self.y.into(), self.z.into());
    }
}
