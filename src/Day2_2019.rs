use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::{FromStr, Chars};
use std::collections::{HashSet, HashMap};

extern crate regex;

use regex::Regex;
use std::ops::Add;

type Grid = HashMap<i32, i32>;

pub fn main() -> Result<(), Error> {
    let mut found:bool = false;
    let mut noun = 0;
    let mut verb = 0;
    while(!found) {
        let mut file = File::open("resources/2019Day2.input")?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        let mut i32s = Grid::new();
        for line in input.lines() {
            let mut vars: Vec<&str> = line.split(",").collect();
            for var in 0..vars.len() {
                println!("{}", var);
                let mut i32Version = vars[var].parse().expect("asdf");
                i32s.insert(var as i32, i32Version);
            }
        }
        i32s.insert(1, noun);
        i32s.insert(2,verb);
        println!("var 0: {}", i32s.len());

        for mut op in (0..i32s.len()).step_by(4) {
            let mut operation = i32s.get(&(op as i32)).expect("");
            println!("op: {}", op);

            if (*operation == 99) {
                break;
            }
            let mut replace = i32s.get(&((op + 3) as i32)).expect("");
            let var1 = i32s.get(&((op + 1) as i32)).expect("");
            let var2 = i32s.get(&((op + 2) as i32)).expect("");


            if (*operation == 1) {
                i32s.insert(*replace, i32s.get(&var1).expect("") + i32s.get(&var2).expect(""));
            } else if (*operation == 2) {
                i32s.insert(*replace, i32s.get(&var1).expect("") * i32s.get(&var2).expect(""));
            } else if ((*operation == 99)) {
                break;
            }
        }
        if(*i32s.get(&0).expect("") == 19690720) {
            println!("noun{}",i32s.get(&1).expect(""));
            println!("verb{}",i32s.get(&2).expect(""));
            println!("sum{}",100*i32s.get(&1).expect("") + i32s.get(&2).expect("") );
            break;

        }
        noun += 1;
        if(noun >= 100) {
            noun = 0;
            verb +=1;
        }

    }


    Ok(())
}