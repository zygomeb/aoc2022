#![allow(unused_imports)]
#![allow(dead_code)]

// module exports functions of format 
// solveXpY, where X is the day, and Y is the part (1 or 2)
// each taking no arguments and returning Result<(), Box<dyn Error>> 

use crate::data::*;
use crate::toy_data::*;

use std::error::Error;
use std::cmp::*;
use std::cmp::Ordering::*;
use std::collections::HashMap;
use core::slice::Iter;
use std::collections::BinaryHeap;

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
        while n < amm*4 {
            let ch = line.chars().nth(n)?;
            if ch != ' ' {
                crane[(n-1)/4].push(ch);
            }
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
        while n < amm*4 {
            let ch = line.chars().nth(n)?;
            if ch != ' ' {
                crane[(n-1)/4].push(ch);
            }
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

pub fn solve6p1() -> Option<()> {
    let b: Vec<char> = DAY6T.chars().collect();
    let mut n: usize = 0;
    for v in 0.. {
        // why yes 
        if b[v] != b[v+1] && b[v+2] != b[v+3] {
            if b[v] != b[v+2] && b[v+1] != b[v+3] {
                if b[v] != b[v+3] && b[v+1] != b[v+2] {
                    n = v+4;
                    break
                }
            }
        }
    }
    println!("{n}");

    Some(())
}

pub fn solve6p2() -> Option<()> {
    let b: Vec<char> = DAY6.chars().collect();
    let mut n: usize = 0;
    'outer: for i in 0.. {
        let mut hm: HashMap<char, bool> = HashMap::new();
        for j in i..=i+13 {
            if let Some(_) = hm.get(&b[j]) {
                break
            }
            hm.insert(b[j], true);
            if j == i+13 {
                n = i+14;
                break 'outer
            }
        }
    }
    println!("{n}");

    Some(())
}

pub fn solve7p1() -> Option<()> {
    #[derive(Debug)]
    struct Node<'a> {
        name: &'a str,
        size: Option<u32>,
        files: Vec<u32>,
        subfolders: Vec<Box<Node<'a>>>
    }

    impl<'a> Node<'_> {
        fn new(name: &str) -> Node {
            Node { name: name, size: None, 
                files: Vec::new(), subfolders: Vec::new() }
        }
    }

    #[derive(Debug)]
    enum Entry<'a> {
        Dir(&'a str),
        File(u32, &'a str) 
    }

    #[derive(Debug)]
    enum Cmd<'a> {
        Ls(Vec<Entry<'a>>),
        Cdb, // cd back
        Cd(&'a str) 
    }

    fn parse_command(cmd: &str) -> Option<Cmd> {
        if cmd.starts_with("ls") {
            let mut entries = Vec::new();
            for entry in cmd.lines().skip(1) {
                if entry.starts_with("dir ") {
                    let (_, n) = entry.split_once(" ")?;
                    entries.push(Entry::Dir(n));
                } else {
                    let (s, n) = entry.split_once(" ")?;
                    entries.push(Entry::File(s.parse().ok()?, n));
                }
            }
            return Some(Cmd::Ls(entries))
        } else if cmd.starts_with("cd ..") {
            return Some(Cmd::Cdb)
        } else {
            return Some(Cmd::Cd(cmd.strip_prefix("cd ")?.trim()))
        }
    }

    fn interpret<'a>(
        node: &mut Box<Node<'a>>, 
        cmds: &mut Vec<Cmd<'a>>) -> Option<()> {

        loop {
            let cmd = cmds.pop()?;
            match cmd {
                Cmd::Cdb => return Some(()),
                Cmd::Cd(s) => {
                    let mut i = 0;
                    loop {
                        if node.subfolders[i].name == s {
                            interpret(&mut node.subfolders[i], cmds)?;
                            break
                        }
                        i += 1;
                    }
                },
                Cmd::Ls(s) => {
                    for entry in s {
                        match entry {
                            Entry::Dir(s) => 
                                node.subfolders
                                    .push(Box::new(Node::new(s))),
                            Entry::File(n, _) => 
                                node.files.push(n),
                        }
                    }
                }
            }
        }
    }

    fn compute(node: &mut Box<Node>) -> u32 {
        let mut tally: u32 = node.files.iter().sum();

        for i in 0..node.subfolders.len() {
            tally += compute(&mut node.subfolders[i]);
        }

        node.size = Some(tally);

        return tally
    }

    fn result(node: Box<Node>) -> u32 {
        let mut tally: u32 = 0;
        for sub in node.subfolders {
            tally += result(sub);
        }

        if let Some(s) = node.size { 
            if s <= 100000 {
                tally += s;
            }
        }

        return tally
    }

    let mut data: Vec<Cmd> = DAY7.split("$ ").skip(2) 
        .map(&parse_command)
        .flatten()
        .collect();

    data.reverse();

    let mut node = Box::new(Node::new("/"));
    interpret(&mut node, &mut data);
    compute(&mut node);

    println!("{:?}", node);
    println!("{:?}", result(node));

    Some(())
}

pub fn solve7p2() -> Option<()> {
    #[derive(Debug)]
    struct Node<'a> {
        name: &'a str,
        size: Option<u32>,
        files: Vec<u32>,
        subfolders: Vec<Box<Node<'a>>>
    }

    impl<'a> Node<'_> {
        fn new(name: &str) -> Node {
            Node { name: name, size: None, 
                files: Vec::new(), subfolders: Vec::new() }
        }
    }

    #[derive(Debug)]
    enum Entry<'a> {
        Dir(&'a str),
        File(u32, &'a str) 
    }

    #[derive(Debug)]
    enum Cmd<'a> {
        Ls(Vec<Entry<'a>>),
        Cdb, // cd back
        Cd(&'a str) 
    }

    struct NodePtr {
        size_ptr: *mut Option<u32>
    }

    fn parse_command(cmd: &str) -> Option<Cmd> {
        if cmd.starts_with("ls") {
            let mut entries = Vec::new();
            for entry in cmd.lines().skip(1) {
                if entry.starts_with("dir ") {
                    let (_, n) = entry.split_once(" ")?;
                    entries.push(Entry::Dir(n));
                } else {
                    let (s, n) = entry.split_once(" ")?;
                    entries.push(Entry::File(s.parse().ok()?, n));
                }
            }
            return Some(Cmd::Ls(entries))
        } else if cmd.starts_with("cd ..") {
            return Some(Cmd::Cdb)
        } else {
            return Some(Cmd::Cd(cmd.strip_prefix("cd ")?.trim()))
        }
    }

    fn interpret<'a>(
        node: &mut Box<Node<'a>>, 
        cmds: &mut Vec<Cmd<'a>>) -> Option<()> {

        loop {
            let cmd = cmds.pop()?;
            match cmd {
                Cmd::Cdb => return Some(()),
                Cmd::Cd(s) => {
                    let mut i = 0;
                    loop {
                        if node.subfolders[i].name == s {
                            interpret(&mut node.subfolders[i], cmds)?;
                            break
                        }
                        i += 1;
                    }
                },
                Cmd::Ls(s) => {
                    for entry in s {
                        match entry {
                            Entry::Dir(s) => 
                                node.subfolders
                                    .push(Box::new(Node::new(s))),
                            Entry::File(n, _) => 
                                node.files.push(n),
                        }
                    }
                }
            }
        }
    }

    fn compute(node: &mut Box<Node>) -> u32 {
        let mut tally: u32 = node.files.iter().sum();

        for i in 0..node.subfolders.len() {
            tally += compute(&mut node.subfolders[i]);
        }

        node.size = Some(tally);

        return tally
    }

    fn result(taken: u32, node: &Box<Node>) -> u32 {
        let mut smallest: u32 = 70000000;

        if let Some(s) = node.size { 
            if taken-s <= 40000000 {
                smallest = s;
            }
        }

        for sub in &node.subfolders {
            smallest = min(smallest, result(taken, sub));
        }

        return smallest
    }

    let mut data: Vec<Cmd> = DAY7.split("$ ").skip(2) 
        .map(&parse_command)
        .flatten()
        .collect();
    data.reverse(); 


    let mut node = Box::new(Node::new("/"));
    interpret(&mut node, &mut data);
    compute(&mut node);

    let taken = node.size.unwrap();

    println!("{:?}", result(taken, &node));

    Some(())
}

pub fn solve8p1() -> Option<()> {
    let mut data: Vec<Vec<(i32, bool)>> = DAY8.lines()
        .map(|x| 
            x.chars().map(|x| 
                (x.to_digit(10).expect("bad data") as i32, false))
            .collect())
        .collect();

    let mut count: u32 = 0;

    let ym = data.len();
    let xm = data[0].len();

    for y in 0..ym {
        let mut biggest: i32 = -1;
        for x in 0..xm {
            if data[y][x].0 > biggest {
                count += !data[y][x].1 as u32;
                data[y][x] = (data[y][x].0, true);
                biggest = data[y][x].0;
            }
        }
    }

    for y in 0..ym {
        let mut biggest: i32 = -1;
        for x in (0..xm).rev() {
            if data[y][x].0 > biggest {
                count += !data[y][x].1 as u32;
                data[y][x] = (data[y][x].0, true);
                biggest = data[y][x].0;
            }
        }
    }

    for x in 0..xm {
        let mut biggest: i32 = -1;
        for y in 0..ym {
            if data[y][x].0 > biggest {
                count += !data[y][x].1 as u32;
                data[y][x] = (data[y][x].0, true);
                biggest = data[y][x].0;
            }
        }
    }

    for x in 0..xm {
        let mut biggest: i32 = -1;
        for y in (0..ym).rev() {
            if data[y][x].0 > biggest {
                count += !data[y][x].1 as u32;
                data[y][x] = (data[y][x].0, true);
                biggest = data[y][x].0;
            }
        }
    }

    println!("{:?}", count);

    Some(())
}

pub fn solve8p2() -> Option<()> {
    let data: Vec<Vec<u32>> = DAY8.lines()
        .map(|x| 
            x.chars().map(|x| 
                x.to_digit(10).expect("bad data"))
            .collect())
        .collect();

    let mut biggest: usize = 0;

    let ym = data.len();
    let xm = data[0].len();

    for y in 1..ym-1 {
        for x in 1..xm-1 {
            let this = data[y][x];
            let mut mul = 1;

            let mut k = 1;
            while k < x && data[y][x-k] < this {
                k += 1;
            }
            mul *= k;

            let mut k = 1;
            while x+k < xm-1 && data[y][x+k] < this {
                k += 1;
            }
            mul *= k;

            let mut k = 1;
            while k < y && data[y-k][x] < this {
                k += 1;
            }
            mul *= k;

            let mut k = 1;
            while y+k < ym-1 && data[y+k][x] < this {
                k += 1;
            }
            mul *= k;

            biggest = max(biggest, mul);
        }
    }

    println!("{}", biggest);

    Some(())
}