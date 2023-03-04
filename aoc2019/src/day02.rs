use std::io::{BufRead};

use crate::error::Error;

pub fn run<R>(_: R) -> Result<(), Error>
where
R: BufRead,
{
    unimplemented!();
}