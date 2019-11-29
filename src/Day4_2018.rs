use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::{FromStr, Chars};
use std::collections::{HashSet, HashMap};
extern crate regex;
use regex::Regex;

type Grid = HashMap<(i32,i32), i32>;
type Grid2 = HashMap<i32, i32>;


pub fn main() -> Result<(),Error> {
    let mut file = File::open("resources/2018Day4.input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let mut grid = Grid::new();
    let re = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\]( Guard #(\d+))?").unwrap();
    let mut latestGuardNumber:i32 = 0;
    let mut latestStartMin:i32 = 0;
    let mut latestWokeMin:i32 = 0;

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            if(line.contains("Guard #")) {
                latestGuardNumber = cap[7].parse().expect("asdf"); 
            }
            if(line.contains("falls")) {
                latestStartMin = cap[5].parse().expect("asdf");
            }
            if(line.contains("wakes")) {
                latestWokeMin = cap[5].parse().expect("asdf");
                for i in latestStartMin..latestWokeMin {
                    let mut x = grid.get(&(latestGuardNumber, i));
                    match x {
                        Some(x) => grid.insert((latestGuardNumber, i), *x + 1),
                        None => grid.insert((latestGuardNumber, i), 1)
                    };
                }
            }
        }
    }
    let mut grid2 = Grid2::new();

        for (key, value) in &grid {
            let (i,j) = key;
             let mut x = grid2.get(&(i));
                    match x {
                        Some(x) => grid2.insert((*i), *x + *value),
                        None => grid2.insert((*i), *value)
                    };
//            println!("{}", i);

//                if(*value > 1) {
//                   sumOfMoreThan2 += 1
//                }
//                println!("{}",value);
        }
    let mut topGuard = 0;
    let mut topValue = 0;
    for(key, value) in &grid2 {
        if(value > &topValue) {
            topGuard = *key;
            topValue = *value;
        }
    }
    let mut topMinute = 0;
    let mut topMostSlept = 0;
    for (key, value) in &grid {
        let (i,j) = key;
        if(i == &topGuard) {
            if(value > &topMostSlept) {
                topMostSlept = *value;
                topMinute = *j;
            }
        }
    }
    println!("topGuard: {}", topGuard);
    println!("topMintue: {}", topMinute);
    println!("maths: {}", topGuard*topMinute);

    let mut topValue = 0;
    let mut topValueGuard = 0;
    let mut topValueMintue = 0;
    for (key, value) in &grid {
        let (i,j) = key;
        if(value > &topValue) {
            topValue = *value;
            topValueGuard = *i;
            topValueMintue = *j;
        }
    }
    println!("topValue: {}", topValue);
    println!("topValueMinute: {}", topValueMintue);
    println!("topValueGuard: {}", topValueGuard);
    println!("maths2: {}", topValueMintue*topValueGuard);
    Ok(())
}