pub trait Difference {
    type Output;

    fn get_diff(&self, other: &Self) -> Self::Output;
}
