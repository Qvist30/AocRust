use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::{FromStr, Chars};
use std::collections::{HashSet, HashMap};
use std::num;
extern crate regex;

use regex::Regex;
use std::ops::Add;

type Point = (i32,i32);

pub fn main() -> Result<(), Error> {

    let mut file = File::open("resources/2019Day3.input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut vec1:Vec<(i32,i32)> = Vec::new();
    let mut vec2:Vec<(i32,i32)> = Vec::new();

    for line in input.lines() {
        let mut vec: Vec<(i32,i32)> = Vec::new();

        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let mut vars: Vec<&str> = line.split(",").collect();
        for var in vars {
            let num:i32 = var[1..].parse().expect("asdf");

            for i in 0..num {
                if var.starts_with("U") {
                    y += 1;
                } else if var.starts_with("L") {
                    x -= 1;
                } else if var.starts_with("R") {
                    x += 1;
                } else {
                    y -= 1;
                }
                vec.push((x,y));
            }
        }
        if vec1.is_empty() {
            vec1 = vec.clone();
        } else {
            vec2 = vec.clone();
        }
    }
    let mut smallestTotal = 9999999;
    for i in 0..vec1.len() {
        for j in 0..vec2.len() {
            if vec1[i].0 == vec2[j].0 && vec1[i].1 == vec2[j].1 {
//                let total = vec1[i].0.abs() + vec1[i].1.abs();
//                if total < smallestTotal {
//                    smallestTotal = total;
//                    println!("smallestTotal: {}", smallestTotal);
//                    println!("intersection: {},{}", vec1[i].0, vec1[i].1);
//                }
                let total = i + j+2;
                if total < smallestTotal {
                    smallestTotal = total;
                    println!("smallestTotal: {}", smallestTotal);
                    println!("intersection: {},{}", vec1[i].0, vec1[i].1);
                }
            }
        }
    }

    Ok(())
}