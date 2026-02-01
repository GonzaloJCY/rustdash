use std::collections::HashMap;

pub fn chunk<T: Clone>(input: &[T], size: usize) -> Vec<Vec<T>> {
    input.chunks(size).map(|chunk| chunk.to_vec()).collect()
}

pub fn compact<T: Clone>(input: &[Option<T>]) -> Vec<T>
where
    T: Clone,
{
    input.iter().filter_map(|x| x.clone()).collect()
}

pub fn flatten_deep<T: Clone>(input: &Vec<Vec<T>>) -> Vec<T> {
    input.iter().flatten().cloned().collect()
}

pub fn unique<T: Clone + std::cmp::Eq + std::hash::Hash>(input: &[T]) -> Vec<T> {
    let mut seen = std::collections::HashSet::new();
    input
        .iter()
        .cloned()
        .filter(|x| seen.insert(x.clone()))
        .collect()
}

pub fn group_by<T, K>(input: &[T], f: impl Fn(&T) -> K) -> HashMap<K, Vec<&T>>
where
    K: Eq + std::hash::Hash,
{
    let mut map: HashMap<K, Vec<&T>> = HashMap::<K, Vec<&T>>::new();
    for item in input {
        let key = f(item);
        map.entry(key).or_default().push(item);
    }
    map
}

pub fn map<T, U>(input: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    input.iter().map(f).collect()
}

pub fn filter<T>(input: &[T], f: impl Fn(&T) -> bool) -> Vec<&T> {
    input.iter().filter(|x| f(x)).collect()
}

pub fn find<T>(input: &[T], f: impl Fn(&T) -> bool) -> Option<&T> {
    input.iter().find(|x| f(x))
}

pub fn sort_by<T, U>(input: &[T], f: impl Fn(&T) -> U) -> Vec<&T>
where
    U: std::cmp::Ord,
{
    let mut array: Vec<&T> = input.iter().collect();
    array.sort_by_key(|x| f(x));
    array
}

pub fn reduce<T, U>(input: &[T], f: impl Fn(U, &T) -> U, initial: U) -> U {
    input.iter().fold(initial, f)
}

pub fn zip<T, U>(a: &[T], b: &[U]) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect()
}

pub fn intersection<T: Eq>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Clone,
{
    //do not place duplicates

    let mut result: Vec<T> = a.iter().filter(|x| b.contains(x)).cloned().collect();

    //remove duplicates
    result.dedup();

    result
}
