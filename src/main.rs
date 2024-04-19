use std::fs;

mod MsbtCpp;
use crate::MsbtCpp::{binary_to_string_rust, MsbtCpp as msbt_cpp};


//remove lib.rs to use this locally
fn main() {
    let file_path = "W:/coding/TotkBits/tmp/PouchContent.msbt";
    let m = msbt_cpp::from_binary_file(file_path).unwrap();
    println!("{}", m.text);
}