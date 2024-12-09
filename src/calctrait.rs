
pub trait CanCalculate {
    type DiffType;
    fn check_threshold(&self, other: &Self, index: i32, threshold: &Self::DiffType) -> bool;
}