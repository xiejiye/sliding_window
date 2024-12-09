use crate::calctrait::CanCalculate;

// 为 i32 实现 CanCalculateChangeRate Trait
impl CanCalculate for i32 {
    type DiffType = i32;
    fn check_threshold(&self, other: &Self, index: i32, threshold: &Self::DiffType) -> bool {
        let tt = (self - other) / index;
        if tt > *threshold {
            return true;
        } else {
            return false;
        }
    }
}

// 为 f64 实现 CanCalculateChangeRate Trait
impl CanCalculate for f64 {
    type DiffType = f64;

    fn check_threshold(&self, other: &Self, index: i32, threshold: &Self::DiffType) -> bool {
        let tt = (self - other).abs() / (index) as f64;
        if tt > *threshold {
            true
        } else {
            false
        }
    }
}
