extern crate itertools;
extern crate rosalind;

use self::itertools::Itertools;
use crate::file;

pub fn handler() {
    let filepath = "files/rosalind_dna_dataset.txt";
    let dna = file::read_file(filepath);
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