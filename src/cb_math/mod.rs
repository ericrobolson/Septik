// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

pub mod cb_matrix;
pub mod cb_range;

extern crate fixed;
use fixed::types::{I20F12, U20F12};

pub type FInt = I20F12;
pub type FUint = U20F12;

/// Returns the number raised to the power
pub fn pow(num: usize, pow: usize) -> usize {
    let mut value = 1;

    for _ in 0..pow {
        value *= num;
    }

    return value;
}

/// NOTE: get rid of floating points as they're non-deterministic
pub fn sqrt_f32(value: f32) -> f32 {
    return value.sqrt();
}

pub fn pow_i32(num: i32, pow: usize) -> i32 {
    let mut value = 1;

    for _ in 0..pow {
        value *= num;
    }

    return value;
}

pub fn index_1d_to_3d(index: usize, x_max: usize, y_max: usize) -> (usize, usize, usize) {
    let mut i = index;

    let z = i / (x_max * y_max);
    i -= (z * x_max * y_max);
    let y = i / x_max;
    let x = i % x_max;

    return (x, y, z);
}

pub fn index_3d_to_1d(x: usize, y: usize, z: usize, grid_size: usize) -> usize {
    let index = x + y * grid_size + z * (grid_size * grid_size);

    if index >= grid_size * grid_size * grid_size {
        return (grid_size * grid_size * grid_size) - 1; // Return the maximum index
    }

    return index;
}

pub fn index_2d_to_1d(x: usize, y: usize, array_size: usize) -> usize {
    return x + array_size * y;
}

pub fn index_1d_to_2d(index: usize, width: usize) -> (usize, usize) {
    return (index % width, index / width);
}

const MAX_NOISE_RES: usize = 32;

pub struct Noise {
    values: Vec<Vec<usize>>,
    max_value: usize,
}

impl Noise {
    /// Create a new noise object. Uses u32's so as to be deterministic across machines
    pub fn new(max_value: usize) -> Self {
        const MIN_VALUE: usize = 0;

        let mut values = vec![vec![]];

        //TODO: interpolate the values
        // populate matrix; making sure the edges are assigned the gradients for the opposing edges (to make it a repeating grid)

        return Self {
            values: values,
            max_value: max_value,
        };
    }

    pub fn at(&self, x: usize, y: usize) -> usize {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // new tests
    #[test]
    fn pow_num0_pow3_returns0() {
        assert_eq!(0, pow(0, 3));
    }

    #[test]
    fn pow_num1_pow3_returns1() {
        assert_eq!(1, pow(1, 3));
    }

    #[test]
    fn pow_num3_pow0_returns1() {
        assert_eq!(1, pow(3, 0));
    }

    #[test]
    fn pow_num2_pow2_returns4() {
        assert_eq!(4, pow(2, 2));
    }

    #[test]
    fn pow_num3_pow3_returns27() {
        assert_eq!(27, pow(3, 3));
    }
}
