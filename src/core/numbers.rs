// sum can take any type that implements the Sum trait (list, array, vector, etc...)

pub fn sum_<T>(iter: &[T]) -> T
where
    T: std::iter::Sum + Copy,
{
    iter.iter().copied().sum()
}

pub fn sum_by_<T, U>(iter: &[T], f: impl Fn(&T) -> U) -> U
where
    U: std::iter::Sum + Copy,
{
    iter.iter().map(f).sum()
}

pub fn max_<T>(slice: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    slice
        .iter()
        .copied()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}

pub fn max_by_<T, U>(slice: &[T], f: impl Fn(&T) -> U) -> Option<&T>
where
    U: Copy + PartialOrd,
{
    slice
        .iter()
        .max_by(|a, b| f(a).partial_cmp(&f(b)).unwrap_or(std::cmp::Ordering::Equal))
}

pub fn min_<T>(slice: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    slice
        .iter()
        .copied()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}
pub fn min_by_<T, U>(slice: &[T], f: impl Fn(&T) -> U) -> Option<&T>
where
    U: Copy + PartialOrd,
{
    slice
        .iter()
        .min_by(|a, b| f(a).partial_cmp(&f(b)).unwrap_or(std::cmp::Ordering::Equal))
}

pub fn mean_<T>(slice: &[T]) -> f64
where
    T: std::convert::Into<f64> + std::iter::Sum + Copy,
{
    sum_(slice).into() / slice.len() as f64
}

pub trait Round {
    type Output;

    fn round_(self, decimals: u32) -> Self::Output;
}

impl Round for f64 {
    type Output = f64;
    fn round_(self, decimals: u32) -> Self::Output {
        (self * 10f64.powf(decimals as f64)).round() / 10f64.powf(decimals as f64)
    }
}

impl Round for &[f64] {
    type Output = Vec<f64>;
    fn round_(self, decimals: u32) -> Self::Output {
        self.iter().map(|x| x.round_(decimals)).collect()
    }
}

pub fn round_<T>(value: T, decimals: u32) -> T::Output
where
    T: Round,
{
    value.round_(decimals)
}
