pub mod debug;
pub mod vector_transforms;

use std::iter::{Zip, Cycle, Skip};

pub fn sequential_pairs_circular<I: Iterator> (it: I) -> Zip<I, Skip<Cycle<I>>>
    where I: Clone
{
    let it_shift = it.clone().cycle().skip(1);
    it.zip(it_shift)
}
