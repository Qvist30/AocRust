use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::{FromStr, Chars};
use std::collections::{HashSet, HashMap};

extern crate regex;

use regex::Regex;

fn getTotalFuel(fuel: i32) -> i32 {
    let mut fuelFuel = (fuel / 3) - 2;
    if fuelFuel <= 0 {
        return fuel;
    } else {
        println!("fuel: {}", fuel);
        println!("fuelFuel: {}", fuelFuel);
        let mut totalFuel = fuel + getTotalFuel(fuelFuel);
        return totalFuel;
    }
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open("resources/2019Day1.input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;


    let mut totalFuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse().expect("asdf");
        let fuel = (mass / 3) - 2;
        let fuelFuel = getTotalFuel(fuel);
        totalFuel += fuelFuel;
        println!("{}", fuelFuel);
    }
    println!("total fuel: {}", totalFuel);
    Ok(())
}