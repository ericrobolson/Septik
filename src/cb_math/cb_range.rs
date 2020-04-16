const MIN_VALUE: i32 = -10000;
const MAX_VALUE: i32 = 10000; // NOT AT TRUE MAX TO PREVENT OVERFLOWS

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CbNormalizedRange {
    pub value: i32,
}

impl CbNormalizedRange {
    pub fn default() -> Self {
        return Self { value: 0 };
    }

    fn scale_value(value: i32, start_min: i32, start_max: i32, end_min: i32, end_max: i32) -> i32 {
        if value == start_min {
            return end_min;
        }

        if value == start_max {
            return end_max;
        }

        let going_to_larger_range = (end_max - end_min) > (start_max - start_min);

        let v;

        let ratio = (end_max - end_min) / (start_max - start_min);
        if going_to_larger_range {
            v = (value - start_min) * ratio + end_min;
        } else {
            let ratio = (start_max - start_min) / (end_max - end_min);
            v = (value - start_min) / ratio + end_min;
        }

        return v;
    }

    pub fn new(value: i32, min_value: i32, max_value: i32) -> Self {
        let v = CbNormalizedRange::scale_value(value, min_value, max_value, MIN_VALUE, MAX_VALUE);

        return Self { value: v };
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }

    pub fn map_to_range_usize(&self, min: usize, max: usize) -> usize {
        // Map to a positive scale, as usizes are positive
        let max = (max as i32) - 1;
        let positive_v =
            CbNormalizedRange::scale_value(self.value, MIN_VALUE, MAX_VALUE, min as i32, max);

        return positive_v as usize;
    }

    pub fn max(&self) -> i32 {
        return MAX_VALUE;
    }

    pub fn min(&self) -> i32 {
        return MIN_VALUE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // new tests
    #[test]
    fn CbNormalizedRange_scale_value_() {
        let start_min = 0;
        let start_max = 1000;
        let value = start_max / 2;

        let end_min = -10;
        let end_max = 10;

        let actual = CbNormalizedRange::scale_value(value, start_min, start_max, end_min, end_max);
        let expected_value = 0;

        assert_eq!(expected_value, actual);
    }

    #[test]
    fn CbNormalizedRange_scale_value_smaller_range_succeeds() {
        let start_min = 0;
        let start_max = 1000;
        let value = start_max / 2;

        let end_min = 0;
        let end_max = 10;

        let actual = CbNormalizedRange::scale_value(value, start_min, start_max, end_min, end_max);
        let expected_value = 5;

        assert_eq!(expected_value, actual);
    }

    #[test]
    fn CbNormalizedRange_scale_value_larger_range_succeeds() {
        let start_min = 0;
        let start_max = 10;
        let value = start_max / 2;

        let end_min = 0;
        let end_max = 1000;

        let actual = CbNormalizedRange::scale_value(value, start_min, start_max, end_min, end_max);
        let expected_value = end_max / 2;

        assert_eq!(expected_value, actual);
    }

    // new tests
    #[test]
    fn CbNormalizedRange_new_from_min0_max1000_value1000_sets_value_to_max() {
        let min = 0;
        let max = 1000;
        let value = max;

        let actual = CbNormalizedRange::new(value, min, max);
        let expected_value = MAX_VALUE;

        assert_eq!(expected_value, actual.value);
    }

    #[test]
    fn CbNormalizedRange_new_from_min0_max1000_value0_sets_value_to_min() {
        let min = 0;
        let max = 1000;
        let value = min;

        let actual = CbNormalizedRange::new(value, min, max);
        let expected_value = MIN_VALUE;

        assert_eq!(expected_value, actual.value);
    }

    #[test]
    fn CbNormalizedRange_new_from_min0_max2_value1_sets_value_to_0() {
        let min = 0;
        let max = 2;
        let value = 1;

        let actual = CbNormalizedRange::new(value, min, max);
        let expected_value = 0;

        assert_eq!(expected_value, actual.value);
    }

    #[test]
    fn CbNormalizedRange_map_to_range_usize_value_is_max_sets_to_usize_max() {
        let min = 0;
        let max = 2;

        let range = CbNormalizedRange { value: MAX_VALUE };

        let actual = range.map_to_range_usize(min, max);
        let expected_value = max - 1;

        assert_eq!(expected_value, actual);
    }

    #[test]
    fn CbNormalizedRange_map_to_range_usize_value_is_min_sets_to_usize_max() {
        let min = 0;
        let max = 10;

        let range = CbNormalizedRange { value: MIN_VALUE };

        let actual = range.map_to_range_usize(min, max);
        let expected_value = min;

        assert_eq!(expected_value, actual);
    }

    #[test]
    fn CbNormalizedRange_map_to_range_usize__value_is_middle_returns_middle_usize() {
        let min = 0;
        let max = 5;

        let range = CbNormalizedRange { value: 0 };

        let actual = range.map_to_range_usize(min, max);
        let expected_value = 2;

        assert_eq!(expected_value, actual);
    }
}
