#![allow(unused_imports)]
#![allow(dead_code)]

// module exports functions of format 
// solveXpY, where X is the day, and Y is the part (1 or 2)
// each taking no arguments and returning Result<(), Box<dyn Error>> 

use crate::data::*;
use crate::toy_data::*;

use std::error::Error;
use std::cmp::Ordering::*;
use std::collections::HashMap;

// use priority_queue::PriorityQueue;

pub fn solve1p1() -> Option<()> {
    let data: Vec<Vec<u32>> = 
        DAY1.split("\n\n").map(|x|
            x.lines().map(|x| x.parse().expect("bad data")).collect())
        .collect();

    // prefer to split parsing and working on data for clarity 
    let results: Option<u32> = 
        data.iter().map(|x| x.iter().sum()).max();
    
    println!("{:?}", results);

    Some(())
}

pub fn solve1p2() -> Option<()> {
    let data: Vec<Vec<u32>> = 
        DAY1.split("\n\n").map(|x|
            x.lines().map(|x| x.parse().expect("bad data")).collect())
        .collect();

    let mut results: Vec<u32> = 
        data.iter().map(|x| x.iter().sum()).collect();
    
    results.sort();
    
    println!("{:?}", results.iter().rev().take(3).sum::<u32>());

    Some(())
}

pub fn solve2p1() -> Option<()> {

    fn score(game: &(u32, u32)) -> u32 {
        let play = game.1;

        let win = match (game.0, game.1) {
            (x, 2) => match x {
                0 => 0,
                1 => 2,
                2 => 1,
                _ => unreachable!("bad data")
            },
            (x, 0) => match x {
                0 => 1,
                1 => 0,
                2 => 2,
                _ => unreachable!("bad data")
            },
            (x, 1) => match x {
                0 => 2,
                1 => 1,
                2 => 0,
                _ => unreachable!("bad data")
            }
            _ => unreachable!("bad data")
        };
        
        play+1 + 3*win
    }

    let mut data: Vec<(u32, u32)> = Vec::new();
    
    for line in DAY2.lines() {
        let t: Vec<&str> = line.split(" ").collect();
        data.push(
            (t[0].chars().nth(0)? as u32 - 'A' as u32, 
            t[1].chars().nth(0)? as u32 - 'X' as u32)
        )
    }

    println!("{}", data.iter().map(score).sum::<u32>());

    Some(())
}

pub fn solve2p2() -> Option<()> {

    fn score(game: &(u32, u32)) -> u32 {

        let play = match (game.0, game.1) {
            (x, 2) => match x {
                0 => 1,
                1 => 2,
                2 => 0,
                _ => unreachable!("bad data")
            },
            (x, 0) => match x {
                0 => 2,
                1 => 0,
                2 => 1,
                _ => unreachable!("bad data")
            },
            (x, 1) => match x {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => unreachable!("bad data")
            }
            _ => unreachable!("bad data")
        };
        
        play+1 + 3*game.1
    }

    let mut data: Vec<(u32, u32)> = Vec::new();
    
    for line in DAY2.lines() {
        let t: Vec<&str> = line.split(" ").collect();
        data.push(
            (t[0].chars().nth(0)? as u32 - 'A' as u32, 
            t[1].chars().nth(0)? as u32 - 'X' as u32)
        )
    }

    println!("{}", data.iter().map(score).sum::<u32>());

    Some(())
}

pub fn solve3p1() -> Option<()> {

    fn priority(x: &char) -> u32 {
        if x.is_ascii_lowercase() {
            *x as u32 - 'a' as u32 + 1
        } else if x.is_ascii_uppercase() {
            *x as u32 - 'A' as u32 + 27
        } else {
            unreachable!("bad data");
        }
    }

    let mut data: Vec<char> = Vec::new();
    
    'outer: for line in DAY3.lines() {
        let (p1, p2) = line.split_at(line.len() / 2);
        let h: HashMap<char, u32> = 
            HashMap::from_iter(p1.chars().map(|x| (x,1)));

        for c in p2.chars() {     
            if let Some(_) = h.get(&c) {
                data.push(c);
                continue 'outer
            }
        }
    }

    println!("{:?}", data.iter().map(&priority).sum::<u32>());
    
    Some(())
}

pub fn solve3p2() -> Option<()> {

    fn priority(x: &char) -> u32 {
        if x.is_ascii_lowercase() {
            *x as u32 - 'a' as u32 + 1
        } else if x.is_ascii_uppercase() {
            *x as u32 - 'A' as u32 + 27
        } else {
            unreachable!("bad data");
        }
    }

    let mut data: Vec<char> = Vec::new();
    
    'outer: for group in DAY3.lines().array_chunks::<3>() {
        let h1: HashMap<char, u32> = 
            HashMap::from_iter(group[0].chars().map(|x| (x,1)));
        let h2: HashMap<char, u32> = 
            HashMap::from_iter(group[1].chars().map(|x| (x,1)));

        for c in group[2].chars() {     
            match (h1.get(&c), h2.get(&c)) {
                (Some(_), Some(_)) => {
                    data.push(c);
                    continue 'outer;
                }
                _ => continue
            }
        }
    }

    println!("{:?}", data.iter().map(&priority).sum::<u32>());
    
    Some(())
}

pub fn solve4p1() -> Option<()> {
    let data: Vec<Vec<u32>> = 
        DAY4.lines()
        .map(|x| x.split(|x| x == '-' || x == ',')
            .map(|x| x.parse().expect("bad data"))
            .collect())
        .collect();
    
    let mut amount: u32 = 0;

    // more efficient not to call collect above
    for pair in data {
        if pair[0] <= pair[2] && pair[1] >= pair[3] {
            amount += 1;
        } else if pair[0] >= pair[2] && pair[1] <= pair[3] {
            amount += 1;
        }
    }

    println!("{:?}", amount);

    Some(())
}

pub fn solve4p2() -> Option<()> {
    let data: Vec<Vec<u32>> = 
        DAY4.lines()
        .map(|x| x.split(|x| x == '-' || x == ',')
            .map(|x| x.parse().expect("bad data"))
            .collect())
        .collect();
    
    let mut amount: u32 = 0;

    // more efficient not to call collect above
    for pair in data {
        if pair[1] >= pair[2] && pair[0] <= pair[2] {
            amount += 1;
        } else if pair[1] >= pair[3] && pair[0] <= pair[3] {
            amount += 1;
        } else if pair[0] >= pair[2] && pair[1] <= pair[3] {
            amount += 1;
        }
    }

    println!("{:?}", amount);

    Some(())
}

pub fn solve5p1() -> Option<()> {
    let (dcargo, dmoves) = DAY5.split_once("\n\n").expect("bad data");
    let mut crane: Vec<Vec<char>> = Vec::new();
    // not going to use and_then because it's not haskell
    let amm: usize = dcargo.lines().last().expect("bad data")
        .chars().rev().find(|x| x.is_numeric()).expect("bad data")
        .to_digit(10).expect("bad data") as usize;
    
    for _ in 1..=amm {
        crane.push(Vec::new());
    } 

    let mut carry = dcargo.lines().rev();
    carry.next();
    for line in carry {
        let mut n: usize = 1;
        // println!("amm: {amm}");
        while n < amm*4 {
            let ch = line.chars().nth(n)?;
            if ch != ' ' {
                crane[(n-1)/4].push(ch);
            }
            // println!("n: {}, ch: {}", n, ch);
            n += 4;
        }
    }
    let lines = dmoves.lines()
        .map(|x| x.chars().filter(|x| x.is_numeric()));

    for line in lines {
        let mut inv = line.clone().rev();
        
        let to: usize = inv.next()?.to_digit(10)?.try_into().unwrap();
        let from: usize = inv.next()?.to_digit(10)?.try_into().unwrap();
        let boxes: usize = inv.rev().collect::<String>().parse().unwrap();
        // println!("{boxes}, {from}, {to}");

        for _ in 1..=boxes {
            let b = crane[from-1].pop()?;
            crane[to-1].push(b);
        }
    }

    for mut stack in crane.into_iter() {
        print!("{}", stack.pop()?);
    }
    println!("");

    Some(())
}

pub fn solve5p2() -> Option<()> {
    let (dcargo, dmoves) = DAY5.split_once("\n\n").expect("bad data");
    let mut crane: Vec<Vec<char>> = Vec::new();
    // not going to use and_then because it's not haskell
    let amm: usize = dcargo.lines().last().expect("bad data")
        .chars().rev().find(|x| x.is_numeric()).expect("bad data")
        .to_digit(10).expect("bad data") as usize;
    
    for _ in 1..=amm {
        crane.push(Vec::new());
    } 

    let mut carry = dcargo.lines().rev();
    carry.next();
    for line in carry {
        let mut n: usize = 1;
        // println!("amm: {amm}");
        while n < amm*4 {
            let ch = line.chars().nth(n)?;
            if ch != ' ' {
                crane[(n-1)/4].push(ch);
            }
            // println!("n: {}, ch: {}", n, ch);
            n += 4;
        }
    }
    let lines = dmoves.lines()
        .map(|x| x.chars().filter(|x| x.is_numeric()));

    for line in lines {
        let mut inv = line.clone().rev();
        
        let to: usize = inv.next()?.to_digit(10)?.try_into().unwrap();
        let from: usize = inv.next()?.to_digit(10)?.try_into().unwrap();
        let boxes: usize = inv.rev().collect::<String>().parse().unwrap();
        // println!("{boxes}, {from}, {to}");

        let mut temp: Vec<char> = Vec::new();
        for _ in 1..=boxes {
            let b = crane[from-1].pop()?;
            temp.push(b);
        }
        for _ in 1..=boxes {
            let b = temp.pop()?;
            crane[to-1].push(b);
        }
    }

    for mut stack in crane.into_iter() {
        print!("{}", stack.pop()?);
    }
    println!("");

    Some(())
}

