extern crate core;

use std::env;
use std::fs;

fn main() {
    let data = get_file();
    let formatted = split_file(&data);
    println!("{:?}", formatted);
}

fn get_file() -> String {
    let content = include_str!("../../input.txt");

    let data = String::from(content);
    if data.chars().count() == 0 {
        panic!("no file");
    }

    return data;
}

fn split_file(file: &str) -> Vec<&str> {
    let split: Vec<&str> = file.split("\n").collect();
    return split;
}
