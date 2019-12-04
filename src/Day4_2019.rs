use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::str::{FromStr, Chars};
use std::collections::{HashSet, HashMap};
use std::num;
extern crate regex;

use regex::Regex;
use std::ops::Add;


pub fn main() -> Result<(), Error> {
    let mut sum = 0;
    for i in 3..8 {
        for j in i..9 {
            for k in j..9 {
                for l in k..9 {
                    for m in l..9 {
                        for n in m..9 {
                            if (((i == j && j!=k) || (j == k && k!=l && i!=j) || (k == l && l!=m && j!=k) || (l == m && m!=n && k!=l) || (m==n && l!=m)) && i <= j && j <= k && k <= l && l <= m && m<=n) {
                                sum += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", sum);
    Ok(())
}