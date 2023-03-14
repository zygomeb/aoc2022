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

pub fn solve9p1() -> Option<()> {
    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    struct Point {
        x: i32,
        y: i32
    }
    #[derive(Debug)]
    struct Plane {
        head: Point, 
        tail: Point,
        visited: HashMap<Point, ()>,
        visited_count: u32
    }
    fn follow_move(plane: &mut Plane) {
        match (
            (plane.head.x - plane.tail.x).abs(), (plane.head.y - plane.tail.y).abs()) {
            
            // diagonal x-escape
            (i,k) if i > 1 && k == 1 => { 
                plane.tail.y = plane.head.y;
                plane.tail.x += (plane.head.x - plane.tail.x) / 2;
            },
            // diagonal y-escape
            (i,k) if i == 1 && k > 1 => {
                plane.tail.x = plane.head.x;
                plane.tail.y += (plane.head.y - plane.tail.y) / 2;
            },
            // horizontal escape
            (2,0) => {
                plane.tail.x += (plane.head.x - plane.tail.x) / 2;
            },
            // vertical escape
            (0,2) => {
                plane.tail.y += (plane.head.y - plane.tail.y) / 2;
            },
            // assumed within reach
            _ => { }

        }
        
        if let None = plane.visited.get(&plane.tail) {
            plane.visited.insert(plane.tail.clone(), ());
            plane.visited_count += 1;
        }
    }

    let mut plane = Plane { 
        head: Point { x: 0, y: 0 }, 
        tail: Point { x: 0, y: 0 },
        visited: HashMap::new(),
        visited_count: 1
    };
    plane.visited.insert(Point { x: 0, y: 0 }, ());

    for line in DAY9.lines() {
        if let Some((dir, num)) = line.rsplit_once(' ') {
            match dir {
                "R" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.head.x += 1;
                        follow_move(&mut plane);
                        println!("head: {:?}, tail: {:?}", &plane.head, &plane.tail);
                    }
                },
                "L" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.head.x -= 1;
                        follow_move(&mut plane);
                        println!("head: {:?}, tail: {:?}", &plane.head, &plane.tail);
                    }
                },
                "U" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.head.y += 1;
                        follow_move(&mut plane);
                        println!("head: {:?}, tail: {:?}", &plane.head, &plane.tail);
                    }
                },
                "D" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.head.y -= 1;
                        follow_move(&mut plane);
                        println!("head: {:?}, tail: {:?}", &plane.head, &plane.tail);
                    }
                },
                _ => unreachable!()
            }
        }
    }

    println!{"{:?}", plane.visited_count};

    Some(())
}

pub fn solve9p2() -> Option<()> {
    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    struct Point {
        x: i32,
        y: i32
    }
    #[derive(Debug)]
    struct Plane {
        rope: [Point; 10], 
        visited: HashMap<Point, ()>,
        visited_count: u32
    }
    fn follow_move(plane: &mut Plane) {
        for n in 0..=8 {
            match (
                (plane.rope[n].x - plane.rope[n+1].x).abs(), 
                (plane.rope[n].y - plane.rope[n+1].y).abs()) {
                
                // diagonal x-escape
                (i,k) if i > 1 && k == 1 => { 
                    plane.rope[n+1].y = plane.rope[n].y;
                    plane.rope[n+1].x += (plane.rope[n].x - plane.rope[n+1].x) / 2;
                },
                // diagonal y-escape
                (i,k) if i == 1 && k > 1 => {
                    plane.rope[n+1].x = plane.rope[n].x;
                    plane.rope[n+1].y += (plane.rope[n].y - plane.rope[n+1].y) / 2;
                },
                // diagonal diagonal escape 
                (2, 2) => {
                    plane.rope[n+1].x += (plane.rope[n].x - plane.rope[n+1].x) / 2;
                    plane.rope[n+1].y += (plane.rope[n].y - plane.rope[n+1].y) / 2;
                },
                // horizontal escape
                (2,0) => {
                    plane.rope[n+1].x += (plane.rope[n].x - plane.rope[n+1].x) / 2;
                },
                // vertical escape
                (0,2) => {
                    plane.rope[n+1].y += (plane.rope[n].y - plane.rope[n+1].y) / 2;
                },
                // assumed within reach
                _ => { }
    
            }
        }
        
        if let None = plane.visited.get(&plane.rope[9]) {
            plane.visited.insert(plane.rope[9].clone(), ());
            plane.visited_count += 1;
        }
    }

    let mut plane = Plane { 
        rope: [
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 }, 
            Point { x: 0, y: 0 } ],

        visited: HashMap::new(),
        visited_count: 1
    };
    plane.visited.insert(Point { x: 0, y: 0 }, ());

    for line in DAY9.lines() {
        if let Some((dir, num)) = line.rsplit_once(' ') {
            match dir {
                "R" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.rope[0].x += 1;
                        follow_move(&mut plane);
                    }
                },
                "L" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.rope[0].x -= 1;
                        follow_move(&mut plane);
                    }
                },
                "U" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.rope[0].y += 1;
                        follow_move(&mut plane);
                    }
                },
                "D" => {
                    for _ in 0..num.parse().expect("bad data") {
                        plane.rope[0].y -= 1;
                        follow_move(&mut plane);
                    }
                },
                _ => unreachable!()
            }
        }
    }

    println!{"{:?}", plane.visited_count};
    Some(())
}

pub fn solve10p1() -> Option<()> {
    let mut x: i32 = 1;
    let mut clock = 1;

    let mut result = 0;

    for line in DAY10.lines() {
        if line.chars().nth(0)? == 'n' {
            clock += 1;
            if (clock - 20) % 40 == 0 {
                result += x * clock;
            }
        } else {
            clock += 1;
            if (clock - 20) % 40 == 0 {
                result += x * clock;
            }

            x += line.split(" ").nth(1)?.parse::<i32>().expect("bad data");
            clock += 1;
            if (clock - 20) % 40 == 0 {
                result += x * clock;
            }
        }
    }

    println!("{result}");

    Some(())
}

pub fn solve10p2() -> Option<()> {
    let mut x: i32 = 1;
    let mut clock = 1;
    fn tick(clock: &mut i32, x: i32) {
        if (*clock%40 - x).abs() < 2 {
            print!("#");
        } else {
            print!(".");
        }
        if *clock%40 == 0 {
            println!("");
        }

        *clock += 1;
    }

    for line in DAY10.lines() {
        if line.chars().nth(0)? == 'n' {
            tick(&mut clock, x);
        } else {
            tick(&mut clock, x);
            x += line.split(" ").nth(1)?.parse::<i32>().expect("bad data");
            tick(&mut clock, x);
        }
    }

    Some(())
}

pub fn solve11p1() -> Option<()> {
    struct Monkey {
        items: Vec<u32>,
        op: Box<dyn Fn(u32) -> u32>,
        test: (u32, usize, usize),
        inspect_count: u32
    }

    let mut monkeys: Vec<Monkey> = Vec::new();

    for monke in DAY11.lines().chain("\n".lines())
        .array_chunks::<7>() {
        
        let items = monke[1].strip_prefix("Starting items: ")?
            .split(", ").map(
                |x| x.parse::<u32>().expect("bad data"))
            .collect();

        let op: Box<dyn Fn(u32) -> u32> = 
            match monke[2]
                .strip_prefix("Operation: new = old ")?
                .split_once(" ")? {
            
            (_, s) if s.parse::<i32>().is_err() => 
                Box::new(|x| x*x),
            ("*", n) => 
                Box::new(|x| x*n.parse::<u32>()
                    .expect("bad data")),
            ("+", n) => 
                Box::new(|x| x+n.parse::<u32>()
                    .expect("bad data")),
            _ => unreachable!()
        };

        let test = 
            (
            monke[3].strip_prefix("Test: divisible by ")?
                .parse().expect("bad data"),
            monke[4].strip_prefix("  If true: throw to monkey ")?
                .parse().expect("bad data"),
            monke[5].strip_prefix("  If false: throw to monkey ")?
                .parse().expect("bad data")
            );

        monkeys.push(
            Monkey { items, op, test, inspect_count: 0 });
    }

    let l = monkeys.len();

    for _ in 0..20 {
        for i in 0..l {
            for item in monkeys[i].items.clone() {
                monkeys[i].inspect_count += 1;

                let item = (monkeys[i].op)(item) / 3;
                if item % monkeys[i].test.0 == 0 {
                    let to = monkeys[i].test.1;
                    monkeys[to].items.push(item);
                } else {
                    let to = monkeys[i].test.2;
                    monkeys[to].items.push(item);
                }
            }
            monkeys[i].items = Vec::new();
        }
    }

    let mut monkey_business = 1;

    monkeys.sort_by_key(|x| x.inspect_count);
    for monkey in monkeys.iter().rev().take(2) {
        monkey_business *= monkey.inspect_count;
    }
    println!("{:?}", monkey_business);

    Some(())
}

pub fn solve11p2() -> Option<()> {
    struct Monkey {
        items: Vec<u64>,
        op: Box<dyn Fn(u64) -> u64>,
        test: (u64, usize, usize),
        inspect_count: u64
    }

    let mut monkeys: Vec<Monkey> = Vec::new();

    for monke in DAY11.lines().chain("\n".lines())
        .array_chunks::<7>() {
        
        let items = monke[1].strip_prefix("Starting items: ")?
            .split(", ").map(
                |x| x.parse::<u64>().expect("bad data"))
            .collect();

        let op: Box<dyn Fn(u64) -> u64> = 
            match monke[2]
                .strip_prefix("Operation: new = old ")?
                .split_once(" ")? {
            
            (_, s) if s.parse::<i32>().is_err() => 
                Box::new(|x| x*x),
            ("*", n) => 
                Box::new(|x| x*n.parse::<u64>()
                    .expect("bad data")),
            ("+", n) => 
                Box::new(|x| x+n.parse::<u64>()
                    .expect("bad data")),
            _ => unreachable!()
        };

        let test = 
            (
            monke[3].strip_prefix("Test: divisible by ")?
                .parse().expect("bad data"),
            monke[4].strip_prefix("  If true: throw to monkey ")?
                .parse().expect("bad data"),
            monke[5].strip_prefix("  If false: throw to monkey ")?
                .parse().expect("bad data")
            );

        monkeys.push(
            Monkey { items, op, test, inspect_count: 0 });
    }

    let l = monkeys.len();
    let cm = monkeys.iter().fold(1, |acc, x| x.test.0 * acc);
  
    for _ in 0..10000 {
        for i in 0..l {
            for item in monkeys[i].items.clone() {
                monkeys[i].inspect_count += 1;

                let item = (monkeys[i].op)(item) % cm;
                if item % monkeys[i].test.0 == 0 {
                    let to = monkeys[i].test.1;
                    monkeys[to].items.push(item);
                } else {
                    let to = monkeys[i].test.2;
                    monkeys[to].items.push(item);
                }
            }
            monkeys[i].items = Vec::new();
        }
    }

    let mut monkey_business = 1;

    monkeys.sort_by_key(|x| x.inspect_count);
    for monkey in monkeys.iter().rev().take(2) {
        monkey_business *= monkey.inspect_count;
    }
    println!("{:?}", monkey_business);

    Some(())
}

pub fn solve12p1() -> Option<()> {
    // 159 x 41

    let data1 = DAY12.lines()
        .map(|x| x.chars().enumerate())
        .enumerate();

    #[derive(Eq, PartialEq, Debug)]
    struct Node {
        distance: u32,
        x: usize,
        y: usize
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            // reverse impl for min-heap
            other.distance.cmp(&self.distance)
        }
    }

    fn h(x: char) -> u32 {
        if x == 'S' {
            0
        } else if x == 'E' {
            'z' as u32 - 'a' as u32
        } else {
            x as u32 - 'a' as u32
        }
    }

    fn cli_diff(this: u32, other: u32) -> u32 {
        if this >= other {
            0
        } else {
            other-this
        }
    }

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();

    let mut start = (0,0);
    'outer: for (y, l) in data1 {
        for (x, c) in l {
            if c == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    let mut data: Vec<Vec<(char, bool)>> = 
        DAY12.lines().map(
            |x| x.chars().map(|c| (c,false)).collect()
        ).collect();

    let my = data.len() - 1;
    let mx = data[0].len() - 1;

    queue.push(Node{distance:0, x:start.0, y:start.1});

    let mut result = 0;
    while let Some(_) = queue.peek() {
        let this = queue.pop()?;
        if data[this.y][this.x].1 {
            continue
        }
        data[this.y][this.x].1 = true;

        let val = data[this.y][this.x];

        if val.0 == 'E' {
            result = this.distance;
            break
        }

        if !data[min(this.y, my-1)+1][this.x].1
            && 
            cli_diff(h(val.0),
            h(data[min(this.y+1, my)][this.x].0)) <= 1 
            {
            
            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x, 
                    y:this.y+1});
        }

        
        if !data[max(this.y, 1)-1][this.x].1 
            && 
            cli_diff(h(val.0),
            h(data[max(this.y-1, 0)][this.x].0)) <= 1
            {

            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x, 
                    y:this.y-1});
        }
        if !data[this.y][min(this.x, mx-1)+1].1
            && 
            cli_diff(h(val.0),
            h(data[this.y][min(this.x+1, mx)].0)) <= 1
            {

            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x+1, 
                    y:this.y});
        }
        if !data[this.y][max(this.x, 1)-1].1
            &&
            cli_diff(h(val.0),
            h(data[this.y][max(this.x-1, 0)].0)) <= 1
            {

            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x-1, 
                    y:this.y});
        }

    }

    println!("{:?}", result);


    Some(())
}

pub fn solve12p2() -> Option<()> {
    // 159 x 41

    let data1 = DAY12.lines()
        .map(|x| x.chars().enumerate())
        .enumerate();

    #[derive(Eq, PartialEq, Debug)]
    struct Node {
        distance: u32,
        x: usize,
        y: usize
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            // reverse impl for min-heap
            other.distance.cmp(&self.distance)
        }
    }

    fn h(x: char) -> u32 {
        if x == 'S' {
            0
        } else if x == 'E' {
            'z' as u32 - 'a' as u32
        } else {
            x as u32 - 'a' as u32
        }
    }

    fn cli_diff(this: u32, other: u32) -> u32 {
        if this <= other {
            0
        } else {
            this-other
        }
    }

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();

    let mut start = (0,0);
    'outer: for (y, l) in data1 {
        for (x, c) in l {
            if c == 'E' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    let mut data: Vec<Vec<(char, bool)>> = 
        DAY12.lines().map(
            |x| x.chars().map(|c| (c,false)).collect()
        ).collect();

    let my = data.len() - 1;
    let mx = data[0].len() - 1;

    queue.push(Node{distance:0, x:start.0, y:start.1});

    let mut result = 0;
    while let Some(_) = queue.peek() {
        let this = queue.pop()?;
        if data[this.y][this.x].1 {
            continue
        }
        data[this.y][this.x].1 = true;

        let val = data[this.y][this.x];

        if val.0 == 'a' {
            result = this.distance;
            break
        }

        if !data[min(this.y, my-1)+1][this.x].1
            && 
            cli_diff(h(val.0),
            h(data[min(this.y+1, my)][this.x].0)) <= 1 
            {
            
            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x, 
                    y:this.y+1});
        }

        
        if !data[max(this.y, 1)-1][this.x].1 
            && 
            cli_diff(h(val.0),
            h(data[max(this.y-1, 0)][this.x].0)) <= 1
            {

            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x, 
                    y:this.y-1});
        }
        if !data[this.y][min(this.x, mx-1)+1].1
            && 
            cli_diff(h(val.0),
            h(data[this.y][min(this.x+1, mx)].0)) <= 1
            {

            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x+1, 
                    y:this.y});
        }
        if !data[this.y][max(this.x, 1)-1].1
            &&
            cli_diff(h(val.0),
            h(data[this.y][max(this.x-1, 0)].0)) <= 1
            {

            queue.push(
                Node{distance:this.distance+1, 
                    x:this.x-1, 
                    y:this.y});
        }

    }

    println!("{:?}", result);


    Some(())
}

pub fn solve13p1() -> Option<()> {
    #[derive(Debug, Clone)]
    enum Element {
        Number(u32),
        List(Vec<Element>)
    }
    use Element::*;

    fn build_ast1(line: &str) -> Vec<Element> {
        let mut ptr = 1;
        build_ast(line, &mut ptr)
    }
    fn build_ast(line: &str, ptr: &mut usize) 
        -> Vec<Element> {

        let mut payload = Vec::new();
        
        while let Some(token) = line.chars().nth(*ptr) {
            *ptr += 1;
            match token {
                ']' => return payload,
                ',' => continue,
                '[' => 
                    payload.push(
                        List(
                            build_ast(line, ptr))),
                n if n.is_digit(10) => {
                    // ugly hack ikr
                    let c = line.chars().nth(*ptr);
                    if c.is_some() && c.unwrap().is_digit(10)  {
                            payload.push(Number(
                                line.get(*ptr-1..*ptr+1)
                                .unwrap()
                                .parse()
                                .unwrap()
                            ));
                            *ptr += 1;
                    } else {
                    payload.push(
                        Number(
                            n as u32 - '0' as u32))
                    }},

                _ => unreachable!()
            }
        }

        unreachable!();
    }

    fn compare(left: &Vec<Element>, 
            right: &Vec<Element>)
        -> Option<bool> {

        let mut idx = 0;

        loop {
            match (left.get(idx), right.get(idx)) {
                (Some(l), Some(r)) => {
                    match (l, r) {
                        (Number(x), Number(y)) => 
                            match x.cmp(y) {
                                Less => return Some(true),
                                Greater => return Some(false),
                                Equal => {}
                            },
                        (List(x), List(y)) => 
                            if let Some(p) = compare(&x, &y) {
                                return Some(p);
                            },
                        (Number(x), List(y)) =>
                            if let Some(p) = compare(
                                &vec![Number(*x)], &y) {
                                    
                                return Some(p);
                            },
                        (List(x), Number(y)) =>
                            if let Some(p) = compare(&x, 
                                &vec![Number(*y)]) {
                                    
                                return Some(p);
                            }
                    }
                },
                (None, Some(_)) => return Some(true),
                (Some(_), None) => return Some(false),
                (None, None) => return None
            }
            idx += 1;
        }
    }

    let data: usize = DAY13.split("\n\n")
        .map(|x| x.split_once("\n").unwrap())
        .map(|(x,y)| (build_ast1(x), build_ast1(y)))
        .map(|(x,y)| compare(&x,&y).unwrap())
        .enumerate()
        .map(|(p,q)| (p+1) * q as usize)
        .sum();

    println!("{:?}", data);

    Some(()) 
}

pub fn solve13p2() -> Option<()> {
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Element {
        Number(u32),
        List(Vec<Element>)
    }
    use Element::*;

    fn build_ast1(line: &str) -> Vec<Element> {
        let mut ptr = 1;
        build_ast(line, &mut ptr)
    }
    fn build_ast(line: &str, ptr: &mut usize) 
        -> Vec<Element> {

        let mut payload = Vec::new();
        
        while let Some(token) = line.chars().nth(*ptr) {
            *ptr += 1;
            match token {
                ']' => return payload,
                ',' => continue,
                '[' => 
                    payload.push(
                        List(
                            build_ast(line, ptr))),
                n if n.is_digit(10) => {
                    // ugly hack ikr
                    let c = line.chars().nth(*ptr);
                    if c.is_some() && c.unwrap().is_digit(10)  {
                            payload.push(Number(
                                line.get(*ptr-1..*ptr+1)
                                .unwrap()
                                .parse()
                                .unwrap()
                            ));
                            *ptr += 1;
                    } else {
                    payload.push(
                        Number(
                            n as u32 - '0' as u32))
                    }},

                _ => unreachable!()
            }
        }

        unreachable!();
    }
    fn compare1(left: &Vec<Element>, 
                right: &Vec<Element>) 
        -> Ordering {

        match compare(left, right) {
            Some(p) => if p { return Less } else { return Greater },
            _ => return Equal
        }
    }
    fn compare(left: &Vec<Element>, 
            right: &Vec<Element>)
        -> Option<bool> {

        let mut idx = 0;

        loop {
            match (left.get(idx), right.get(idx)) {
                (Some(l), Some(r)) => {
                    match (l, r) {
                        (Number(x), Number(y)) => 
                            match x.cmp(y) {
                                Less => return Some(true),
                                Greater => return Some(false),
                                Equal => {}
                            },
                        (List(x), List(y)) => 
                            if let Some(p) = compare(&x, &y) {
                                return Some(p);
                            },
                        (Number(x), List(y)) =>
                            if let Some(p) = compare(
                                &vec![Number(*x)], &y) {
                                    
                                return Some(p);
                            },
                        (List(x), Number(y)) =>
                            if let Some(p) = compare(&x, 
                                &vec![Number(*y)]) {
                                    
                                return Some(p);
                            }
                    }
                },
                (None, Some(_)) => return Some(true),
                (Some(_), None) => return Some(false),
                (None, None) => return None
            }
            idx += 1;
        }
    }

    let mut data: Vec<Vec<Element>> = DAY13.split("\n\n")
        .flat_map(|x| x.split("\n"))
        .chain(["[[2]]", "[[6]]"].into_iter())
        .map(|x| build_ast1(x))
        .collect();
    
    data[..].sort_by(compare1);

    let l1 = data.iter()
        .position(|x| *x == build_ast1("[[2]]"))
        .unwrap();

    let l2 = data.iter()
        .position(|x| *x == build_ast1("[[6]]"))
        .unwrap();

    println!("{:?}", (l1+1) * (l2+1));

    Some(()) 
}

pub fn solve14p1() -> Option<()> {
    let mut screen: [[char; 700]; 200] = [['.'; 700]; 200];
    fn display(scr: &[[char; 700]; 200]) {
        for y in 0..170 {
            for x in 460..570 {
                print!("{}", scr[y][x]);
            }
            print!("\n");
        }
    }

    fn drop(scr: &mut [[char; 700]; 200]) -> bool {
        let mut pos = (500, 0);
        loop {
            if pos.1 > 198 {
                return false
            } 
            if scr[pos.1+1][pos.0] == '.' {
                pos = (pos.0, pos.1+1);
                continue
            } 
            if scr[pos.1+1][pos.0-1] == '.' {
                pos = (pos.0-1, pos.1+1);
                continue
            }
            if scr[pos.1+1][pos.0+1] == '.' {
                pos = (pos.0+1, pos.1+1);
                continue
            }
            scr[pos.1][pos.0] = 'o';
            return true
        }
    }

    for line in DAY14.lines() {
        let mut prev = (1000,0);
        for xy in line.split(" -> ") {
            let (xc,yc) = xy.split_once(',')?;
            let (x,y) = (xc.parse().ok()?, yc.parse().ok()?);
            if prev.0 == 1000 {
                prev = (x,y);
                continue
            }

            if prev.0 == x {
                for i in prev.1..y+1 {
                    screen[i][x] = '#';
                }
                for i in y..prev.1+1 {
                    screen[i][x] = '#';
                }
            } else {
                for i in prev.0..x+1 {
                    screen[y][i] = '#';
                }
                for i in x..prev.0+1 {
                    screen[y][i] = '#';
                }
            }
            prev = (x,y);
        }
    }

    let mut count = 0;
    while drop(&mut screen) {
        count += 1;
    }
    display(&screen);
    
    println!("{:?}", count);
    
    Some(())
}

pub fn solve14p2() -> Option<()> {
    let mut screen: [[char; 700]; 200] = [['.'; 700]; 200];
    fn display(scr: &[[char; 700]; 200]) {
        for y in 0..170 {
            for x in 410..630 {
                print!("{}", scr[y][x]);
            }
            print!("\n");
        }
    }

    fn drop(scr: &mut [[char; 700]; 200]) -> bool {
        let mut pos = (500, 0);
        if scr[0][500] == 'o' {
            return false
        }

        loop {
            if pos.1 > 198 {
                return false
            } 
            if scr[pos.1+1][pos.0] == '.' {
                pos = (pos.0, pos.1+1);
                continue
            } 
            if scr[pos.1+1][pos.0-1] == '.' {
                pos = (pos.0-1, pos.1+1);
                continue
            }
            if scr[pos.1+1][pos.0+1] == '.' {
                pos = (pos.0+1, pos.1+1);
                continue
            }
            scr[pos.1][pos.0] = 'o';
            return true
        }
    }

    let mut highest = 0;

    for line in DAY14.lines() {
        let mut prev = (1000,0);
        for xy in line.split(" -> ") {
            let (xc,yc) = xy.split_once(',')?;
            let (x,y) = (xc.parse().ok()?, yc.parse().ok()?);

            highest = max(y, highest);

            if prev.0 == 1000 {
                prev = (x,y);
                continue
            }

            if prev.0 == x {
                for i in prev.1..y+1 {
                    screen[i][x] = '#';
                }
                for i in y..prev.1+1 {
                    screen[i][x] = '#';
                }
            } else {
                for i in prev.0..x+1 {
                    screen[y][i] = '#';
                }
                for i in x..prev.0+1 {
                    screen[y][i] = '#';
                }
            }
            prev = (x,y);
        }
    }

    highest += 2;
    for x in 0..699 {
        screen[highest][x] = '#';
    }

    let mut count = 0;
    while drop(&mut screen) {
        count += 1;
    }
    display(&screen);
    
    println!("{:?}", count);
    
    Some(())
}