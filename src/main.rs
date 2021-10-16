extern crate itertools;

use itertools::Itertools;
use std::fs;

fn main() {
    let filepath = "files/rosalind_dna_dataset.txt";
    let dna = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");
    let res = dna
        .chars()
        .sorted()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_group, records)| records.count().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", res);
}