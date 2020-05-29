use fixed::types::I20F12;
type fix = I20F12;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FixedNumber {
    value: fix,
}

impl FixedNumber {
    pub fn PI() -> Self {
        Self { value: fix::PI }
    }

    pub fn min(a: Self, b: Self) -> Self {
        if a.value <= b.value {
            return a;
        }

        b
    }

    /// Sine
    pub fn sin(&self) -> Self {
        //TODO: Convert to fixed!
        let v: f32 = (*self).into();

        let sin = v.sin();

        Self::from_f32(sin)
    }

    /// Cosine
    pub fn cos(&self) -> Self {
        //TODO: Convert to fixed!
        let v: f32 = (*self).into();

        let cos = v.cos();

        Self::from_f32(cos)
    }

    pub fn max(a: Self, b: Self) -> Self {
        if a.value <= b.value {
            return b;
        }

        a
    }

    fn from_i32(number: i32) -> Self {
        Self {
            value: fix::from_num(number),
        }
    }

    fn from_f32(number: f32) -> Self {
        Self {
            value: fix::from_num(number),
        }
    }
}

impl std::ops::Add for FixedNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> <Self as std::ops::Add<Self>>::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::Neg for FixedNumber {
    type Output = Self;
    fn neg(self) -> <Self as std::ops::Neg>::Output {
        Self { value: -self.value }
    }
}

impl std::ops::AddAssign for FixedNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl std::ops::Sub for FixedNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> <Self as std::ops::Sub<Self>>::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl std::ops::SubAssign for FixedNumber {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl std::ops::Mul for FixedNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> <Self as std::ops::Mul<Self>>::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl std::ops::MulAssign for FixedNumber {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl std::ops::Div for FixedNumber {
    type Output = Self;
    fn div(self, rhs: Self) -> <Self as std::ops::Div<Self>>::Output {
        Self {
            value: self.value / rhs.value,
        }
    }
}

impl std::ops::DivAssign for FixedNumber {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Into<FixedNumber> for i32 {
    fn into(self) -> FixedNumber {
        FixedNumber::from_i32(self)
    }
}

impl Into<f32> for FixedNumber {
    fn into(self) -> f32 {
        self.value.to_num::<f32>()
    }
}

impl std::convert::From<f32> for FixedNumber {
    fn from(value: f32) -> Self {
        FixedNumber::from_f32(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn FixedNumber_divide() {
        let expected = FixedNumber::from_i32(0);
        let v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(2000);

        assert_eq!(expected, v1 / v2);

        let expected = fix::from_num(2) / fix::from_num(3);

        let v1 = FixedNumber::from_i32(2);
        let v2 = FixedNumber::from_i32(3);

        assert_eq!(expected, (v1 / v2).value);
    }

    #[test]
    fn FixedNumber_divide_assign() {
        let expected = FixedNumber::from_i32(0);
        let mut v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(2000);

        v1 /= v2;

        assert_eq!(expected, v1);

        let expected = fix::from_num(2) / fix::from_num(3);

        let mut v1 = FixedNumber::from_i32(2);
        let v2 = FixedNumber::from_i32(3);

        v1 /= v2;

        assert_eq!(expected, v1.value);
    }

    #[test]
    fn FixedNumber_subtract() {
        let expected = FixedNumber::from_i32(-2000);

        let v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(2000);

        assert_eq!(expected, v1 - v2);

        let expected = FixedNumber::from_i32(2000);

        let v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(-2000);

        assert_eq!(expected, v1 - v2);
    }

    #[test]
    fn FixedNumber_subtract_assign() {
        let expected = FixedNumber::from_i32(-2000);

        let mut v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(2000);

        v1 -= v2;

        assert_eq!(expected, v1);

        let expected = FixedNumber::from_i32(2000);

        let mut v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(-2000);

        v1 -= v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn FixedNumber_add() {
        let expected = FixedNumber::from_i32(2000);

        let v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(2000);

        assert_eq!(expected, v1 + v2);

        let expected = FixedNumber::from_i32(-1);

        let v1 = FixedNumber::from_i32(-2001);
        let v2 = FixedNumber::from_i32(2000);

        assert_eq!(expected, v1 + v2);
    }

    #[test]
    fn FixedNumber_add_assign() {
        let expected = FixedNumber::from_i32(2000);

        let mut v1 = FixedNumber::from_i32(0);
        let v2 = FixedNumber::from_i32(2000);

        v1 += v2;

        assert_eq!(expected, v1);

        let expected = FixedNumber::from_i32(-222);

        let mut v1 = FixedNumber::from_i32(-2222);
        let v2 = FixedNumber::from_i32(2000);

        v1 += v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn FixedNumber_from_i32_0() {
        let num = 0;
        let value = fix::from_num(num);
        let fixed_number = FixedNumber::from_i32(num);

        assert_eq!(value, fixed_number.value);
    }

    #[test]
    fn FixedNumber_from_i32_1001() {
        let num = 1001;
        let value = fix::from_num(num);
        let fixed_number = FixedNumber::from_i32(num);

        assert_eq!(value, fixed_number.value);
    }

    #[test]
    fn FixedNumber_from_i32_n2030() {
        let num = -2030;
        let value = fix::from_num(num);
        let fixed_number = FixedNumber::from_i32(num);

        assert_eq!(value, fixed_number.value);
    }
}
