use std::io::{BufRead, BufReader};

use crate::error::Error;

pub fn run<R>(mut input: R) -> Result<(), Error>
where
R: BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    let mut reader = BufReader::new(&content[..]);
    run_part(&mut reader, part_one)?;

    let mut reader = BufReader::new(&content[..]);
    run_part(&mut reader, part_two)?;

    Ok(())
}

pub fn run_part<F, R>(input: &mut R, func: F) -> Result<(), Error>
where
R: BufRead,
F: Fn(usize) -> usize,
{
    let mut buffer = String::new();

    let mut total = 0;
    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        
        let n = buffer.trim().parse::<usize>()?;

        let fuel = func(n);
        total += fuel;

        buffer.clear();
    }

    println!("{}", total);

    Ok(())
}

fn part_one(size: usize) -> usize {
    match (size / 3).checked_sub(2) {
        Some(m) => m,
        None => 0,
    }
}

fn part_two(mut size: usize) -> usize {
    let mut total = 0;
    loop {
        let m = match (size / 3 ).checked_sub(2) {
            Some(m) => m,
            None => break total,
        };
        total += m;
        size = m;
    }
}