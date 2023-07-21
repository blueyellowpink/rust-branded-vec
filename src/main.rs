use std::marker::PhantomData;

struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

struct BrandedVec<'id, T> {
    vec: Vec<T>,
    _marker: InvariantLifetime<'id>,
}

struct BrandedIndex<'id> {
    index: usize,
    _marker: InvariantLifetime<'id>,
}

fn main() {
    println!("Hello, world!");
}
