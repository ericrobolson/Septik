use fixed::types::I20F12;
type fix = I20F12;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FixedNumber {
    value: fix,
}

impl FixedNumber {
    fn from_i32(number: i32) -> Self {
        return Self {
            value: fix::from_num(number),
        };
    }
}

impl std::ops::Add for FixedNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> <Self as std::ops::Add<Self>>::Output {
        return Self {
            value: self.value + rhs.value,
        };
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
        return Self {
            value: self.value - rhs.value,
        };
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
        return Self {
            value: self.value * rhs.value,
        };
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
        return Self {
            value: self.value / rhs.value,
        };
    }
}

impl std::ops::DivAssign for FixedNumber {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Into<FixedNumber> for i32 {
    fn into(self) -> FixedNumber {
        return FixedNumber::from_i32(self);
    }
}

impl Into<f32> for FixedNumber {
    fn into(self) -> f32 {
        return self.value.to_num::<f32>();
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
