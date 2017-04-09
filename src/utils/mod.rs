pub mod transforms_3d;
pub mod transforms_2d;

use std::iter::{Zip, Cycle, Skip};

pub fn sequential_pairs_circular<I: Iterator> (it: I) -> Zip<I, Skip<Cycle<I>>>
    where I: Clone
{
    let it_shift = it.clone().cycle().skip(1);
    it.zip(it_shift)
}
