use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::FromStr;
use std::collections::HashSet;

pub fn main() -> Result<(),Error> {
    let mut file = File::open("resources/2018Day1.input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let mut i = 0;
    let mut vec = HashSet::new();
    vec.insert(0);
    let mut frequency:Vec<i32> = Vec::new();
//    for line in reader.lines() {
//        let string = line.unwrap();
//        let int:i32 = string.parse().expect("Wanted a number");
//        frequency.push(int);
//        i += int;
//        if(vec.contains(&i)) {
//            println!("{}abc",i);
//            break;
//        }
////        println!("{}",i);
//        vec.push(i);
//
//    }
    loop {
        for line in input.lines() {
            let int:i32 = line.parse().expect("Wanted a number");
            i += int;
            if vec.contains(&i) {
                println!( "{}ddd", i);
                return Ok(());
            }
            vec.insert(i);
        }
        println!("{}c",i);
    }


    println!("{}",i);
    Ok(())
}