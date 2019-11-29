use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::{FromStr, Chars};
use std::collections::{HashSet, HashMap};
extern crate regex;
use regex::Regex;

type Grid = HashMap<(i32, i32), i32>;
pub fn main() -> Result<(),Error> {
    let mut file = File::open("resources/2018Day3.input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut sumOfMoreThan2 = 0;
    let mut grid = Grid::new();
//    loop {
//        let mut j:i32=0;
        for line in input.lines() {
            for cap in re.captures_iter(line){
//                println!("{}", &cap[1]);
//                println!("#{} @ {},{}: {}x{}",&cap[1],&cap[2],&cap[3],&cap[4],&cap[5]);
                let leftCell:i32 = cap[2].parse().expect("asdf");
//                println!("left: {}", leftCell);
                let width:i32 = cap[4].parse().expect("ase");
                let rightCell:i32 = leftCell +  width - 1;
//                println!("right: {}", rightCell);
                let topCell:i32 = cap[3].parse().expect("asdf");
                let height:i32 = cap[5].parse().expect("ase");
//                println!("topCell: {}", topCell);
                let bottomCell:i32 = topCell + height - 1;
//                println!("bottomCell: {}", bottomCell);
                for  i in leftCell..(rightCell+1) {
                   for j in topCell..(bottomCell+1) {
                       let mut x = grid.get(&(i,j));
                        match x {
                            Some(x) => grid.insert((i,j), *x+1),
                            None => grid.insert((i,j), 1)
                        };

                   }
                }
            }

//            j+=1;
        }
    for line in input.lines() {
        for cap in re.captures_iter(line) {
//             println!("#{} @ {},{}: {}x{}",&cap[1],&cap[2],&cap[3],&cap[4],&cap[5]);
                let leftCell:i32 = cap[2].parse().expect("asdf");
//                println!("left: {}", leftCell);
                let width:i32 = cap[4].parse().expect("ase");
                let rightCell:i32 = leftCell +  width - 1;
//                println!("right: {}", rightCell);
                let topCell:i32 = cap[3].parse().expect("asdf");
                let height:i32 = cap[5].parse().expect("ase");
//                println!("topCell: {}", topCell);
                let bottomCell:i32 = topCell + height - 1;
//                println!("bottomCell: {}", bottomCell);
                let mut found = true;
                for  i in leftCell..(rightCell+1) {
                    for j in topCell..(bottomCell + 1) {
                        let value = grid.get(&(i,j)).expect("");
                        if(*value != 1) {
                            found = false;
                            break;
                        }
                    }
                }
            if (found) {
                println!("{}", &cap[1]);
            }
        }
    }
//        for (key, value) in &grid {
//                if(*value > 1) {
//                   sumOfMoreThan2 += 1
//                }
////                println!("{}",value);
//        }
//    println!("{}", sumOfMoreThan2);

    Ok(())
}