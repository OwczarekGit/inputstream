pub trait Difference {
    type Diff;
    fn get_diff(&self, other: &Self) -> Self::Diff;
}
