use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::{FromStr, Chars};
use std::collections::HashSet;

pub fn main() -> Result<(),Error> {
    let mut file = File::open("resources/2018Day2.input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let mut i = 0;
    let mut two_x = 0;
    let mut three_x = 0;
//    loop {
        for line in input.lines() {
            for j in input.lines() {
                let mut foundOne = false;
                let lineChars:Vec<char> = line.chars().collect();
                let j_chars:Vec<char> = j.chars().collect();
                for i in 0..26 {
                    if(lineChars[i] != j_chars[i]) {
                        if(foundOne) {
                            foundOne = false;
                            break;
                        }
                        foundOne = true;
                    }
                }
                if(foundOne) {
                    println!("{}",line);
                    println!("{}",j);
                    break;
                }

            }
//            let bytes = line.chars();
//            let mut num:[char;26] = ['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'];
//            let mut i = 0;
//
//            for b in bytes {
//
//                num[i] = b;
//
//                i+=1;
//            }
//            let v = num;
//            let mut hasTwo = false;
//            let mut hasThree = false;
//            for n in num.iter() {
//                let count = v.iter().filter(|&f|f==n).count();
//                //println!("{}", count);
//                if(count == 2) {
//
//                    hasTwo = true;
//                    println!("2 {}", n);
//                }
//                if(count == 3) {
//                    hasThree = true;
////                    println!("3 {}", n);
//                }
//            }
//            if(hasTwo) {
//                two_x+=1;
//                println!("{}",line);
//                println!("{}", two_x);
//            }
//            if(hasThree) {
//                println!("{}",line);
//                three_x +=1;
//                println!("{}", three_x);
//            }
        }
//        println!("{}c",i);
//    }


    println!("{}",two_x*three_x);
    Ok(())
}