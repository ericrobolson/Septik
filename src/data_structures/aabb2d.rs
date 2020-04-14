#[derive(Copy, Clone)]
pub struct Aabb2d<T> {
    pub x0: T,
    pub y0: T,
    pub x1: T,
    pub y1: T,
}

impl<T> Aabb2d<T>
where
    T: std::cmp::PartialOrd + std::ops::Add<Output = T> + Copy + std::fmt::Display,
{
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self {
            x0: x,
            y0: y,
            x1: x + w,
            y1: y + h,
        }
    }

    pub fn contains(&self, x: &T, y: &T) -> bool {
        let x = *x;
        let y = *y;

        let within_x = self.x0 <= x && x <= self.x1;
        let within_y = self.y0 <= y && y <= self.y1;

        return within_x && within_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb2d_new_sets_to_expected() {
        let aabb = Aabb2d::<i32>::new(0, 0, 1280, 720);

        assert_eq!(0, aabb.x0);
        assert_eq!(0, aabb.y0);
        assert_eq!(1280, aabb.x1);
        assert_eq!(720, aabb.y1);

        let aabb = Aabb2d::<i32>::new(-640, -360, 1280, 720);

        assert_eq!(-640, aabb.x0);
        assert_eq!(-360, aabb.y0);
        assert_eq!(640, aabb.x1);
        assert_eq!(360, aabb.y1);
    }

    #[test]
    fn aabb2d_contains_works_as_expepcted() {
        let aabb = Aabb2d::<i32>::new(0, 0, 1280, 720);

        assert_eq!(true, aabb.contains(&0, &0));
        assert_eq!(true, aabb.contains(&1280, &720));
        // Boundary conditions
        assert_eq!(false, aabb.contains(&0, &-1));
        assert_eq!(false, aabb.contains(&-1, &0));

        assert_eq!(false, aabb.contains(&1281, &0));
        assert_eq!(false, aabb.contains(&0, &721));
    }
}
