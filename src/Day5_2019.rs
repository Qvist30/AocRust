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
//    while(!found) {
        let mut input_id:i32 = 5;
        let mut file = File::open("resources/2019Day5.input")?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        let mut i32s = Grid::new();
        for line in input.lines() {
            let mut vars: Vec<&str> = line.split(",").collect();
            for var in 0..vars.len() {
//                println!("{}", var);
                let mut i32Version = vars[var].parse().expect("asdf");
                i32s.insert(var as i32, i32Version);
            }
        }
//        i32s.insert(1, noun);
//        i32s.insert(2,verb);
//        println!("var 0: {}", i32s.len());
        let mut op = 0;
        let mut prev_op = 99999;
        while op < i32s.len() {
            let mut operation_obj = i32s.get(&(op as i32)).expect("");

            let mut operation = operation_obj % 100;
            let mut firstParamCode:i32 = (operation_obj/100) % 10;
            let mut secondParamCode:i32 = (operation_obj/1000) % 10;
            let mut thirdParamCode:i32 = (operation_obj/10000);
//            println!("first: {}", firstParamCode);
//            println!("second: {}", secondParamCode);
//
//            println!("third: {}", thirdParamCode);

            println!("op: {}", op);
            println!("operation: {}", operation);
            if(prev_op == op) {
                break;
            } else {
                prev_op = op;
            }
            if (operation == 99) {
                break;
            }

            if (operation == 1) {
                let mut replace = i32s.get(&((op + 3) as i32)).expect("");
                let mut var1 = i32s.get(&((op + 1) as i32)).expect("") ;
                let mut var2 = i32s.get(&((op + 2) as i32)).expect("");
                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
                var2 = if secondParamCode == 0 {i32s.get(&var2).expect("") } else {var2};
                println!("replace: {}", *replace);
                println!("var1: {}", var1+var2);
                i32s.insert(*replace,  var1 + var2);
                op+=4;
            } else if (operation == 2) {
                let mut replace = i32s.get(&((op + 3) as i32)).expect("");
                let mut var1 = i32s.get(&((op + 1) as i32)).expect("") ;
                let mut var2 = i32s.get(&((op + 2) as i32)).expect("");

                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
                var2 = if secondParamCode == 0 {i32s.get(&var2).expect("") } else {var2};


                i32s.insert(*replace, var1 * var2);
                op+=4;
            } else if (operation == 3) {
                let mut replace = i32s.get(&((op + 1) as i32)).expect("");
//                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
                println!("replace: {}", *replace);
                println!("input_id: {}", input_id);
                i32s.insert(*replace, input_id);
                op+=2;
            } else if (operation == 4) {
                let mut var1 = i32s.get(&((op + 1) as i32)).expect("");
                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
//                i32s.insert(*var1, input_id);
                println!("diagnosticCode{}", *var1);
                op+=2;
            } else if (operation == 5) {
                let mut var1 = i32s.get(&((op + 1) as i32)).expect("");
                let mut var2 = i32s.get(&((op + 2) as i32)).expect("");
                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
                var2 = if secondParamCode == 0 {i32s.get(&var2).expect("") } else {var2};
                println!("var1: {}", var1);
                println!("var2: {}", var2);
                if(*var1 != 0) {
                    op = *var2 as usize;
                } else {
                    op += 3;
                }
            } else if (operation == 6) {
                let mut var1 = i32s.get(&((op + 1) as i32)).expect("");
                let mut var2 = i32s.get(&((op + 2) as i32)).expect("");
                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
                var2 = if secondParamCode == 0 {i32s.get(&var2).expect("") } else {var2};

                if(*var1 == 0) {
                    op = *var2 as usize;
                } else {
                    op += 3;
                }
            } else if (operation == 7) {
                let mut var1 = i32s.get(&((op + 1) as i32)).expect("");
                let mut var2 = i32s.get(&((op + 2) as i32)).expect("");
                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
                var2 = if secondParamCode == 0 {i32s.get(&var2).expect("") } else {var2};
                let mut replace = i32s.get(&((op + 3) as i32)).expect("");
                if(*var1 < *var2) {
                    i32s.insert(*replace, 1);
                } else {
                     i32s.insert(*replace, 0);
                }
                op += 4;
            } else if (operation == 8) {
                let mut var1 = i32s.get(&((op + 1) as i32)).expect("");
                let mut var2 = i32s.get(&((op + 2) as i32)).expect("");
                var1 = if firstParamCode == 0 {i32s.get(&var1).expect("") } else {var1};
                var2 = if secondParamCode == 0 {i32s.get(&var2).expect("") } else {var2};
                let mut replace = i32s.get(&((op + 3) as i32)).expect("");
                if(*var1 == *var2) {
                    i32s.insert(*replace, 1);
                } else {
                     i32s.insert(*replace, 0);
                }
                op += 4;
            } else if ((operation == 99)) {
                break;
            }
//        }
//        if(*i32s.get(&0).expect("") == 19690720) {
//            println!("noun{}",i32s.get(&1).expect(""));
//            println!("verb{}",i32s.get(&2).expect(""));
//            println!("sum{}",100*i32s.get(&1).expect("") + i32s.get(&2).expect("") );
//            break;
//
//        }
//        noun += 1;
//        if(noun >= 100) {
//            noun = 0;
//            verb +=1;
//        }
//        break;
    }


    Ok(())
}