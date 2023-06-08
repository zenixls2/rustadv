#![feature(iterator_try_collect)]
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::env;
use std::fs;

fn find(target: i64, split: i64) -> Result<(i64, i64)> {
    let sqrt = (target as f64).sqrt() as i64;
    for i in 1..sqrt + 1 {
        let j = split - i;
        if j * i == target {
            return Ok((i, j));
        }
    }
    Err(anyhow!("cannot find answer!"))
}

fn main() -> Result<()> {
    let path: &str = &env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("please provide input.txt path"))?;
    let inputs = fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse::<i64>())
        .try_collect::<HashSet<i64>>()?;
    let mut part1 = vec![];
    let (i, j) = find(866436, 2020)?;
    part1.push(i);
    part1.push(j);
    if part1
        .into_iter()
        .find_map(|i| (!inputs.contains(&i)).then_some(()))
        .is_some()
    {
        return Err(anyhow!("part1 find fail"));
    } else {
        println!("part1 found");
    }

    let mut part2 = vec![];
    let target: i64 = 276650720;
    let sqrt = (target as f64).sqrt() as i64;
    for i in 1..sqrt + 1 {
        let j = 2020 - i;
        if let Ok((k, l)) = find(target / i, j) {
            part2.push(i);
            part2.push(k);
            part2.push(l);
            break;
        }
    }
    if part2
        .into_iter()
        .find_map(|i| (!inputs.contains(&i)).then_some(()))
        .is_some()
    {
        return Err(anyhow!("part2 find fail"));
    } else {
        println!("part2 found");
    }
    Ok(())
}
