use rubikmaster::cfop;
use rubikmaster::matrix::{of, PermutationMatrix};
use rubikmaster::{Command, Move::*};

use clap::Clap;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Clap, Debug)]
#[clap(name = "oll-comb")]
struct Opts {
    #[clap(name = "FILE")]
    file: String,
}

fn parse(s: &str) -> PermutationMatrix {
	todo!()
}
fn main() {
	let opt = Opts::parse();
	let file = File::open(opt.file).unwrap();
	let reader = BufReader::new(file);
	let oll: Vec<String> = serde_json::from_reader(reader).unwrap();
	let mut oll_tbl = HashMap::new();
	for s in oll {
		let mat = parse(&s);
		let h = mat.inv_perm;
		oll_tbl.entry(h).or_insert(vec![]).push(mat);
	}
	let mut id = 0;
	let mut h2i = HashMap::new();
	for k in oll_tbl.keys() {
		id += 1;
		h2i.insert(k, id);
	}
	dbg!(&oll_tbl);
}