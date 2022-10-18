use std::str::FromStr;
use smartcore::numbers::realnum::*;

fn printsig() {
    let sig1 = 1.0.sigmoid();

    println!("{:?}", sig1);     // 0.7310585786300049
}

fn main() {
    printsig();
}