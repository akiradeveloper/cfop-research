use rubikmaster::cfop;
use rubikmaster::matrix::{of, PermutationMatrix};
use rubikmaster::{Command, Move::*};

use clap::Clap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

const PERMS: [(&str, &str); 2] = [
    ("Ub", "R2U RUR' U'R'U' R'UR'"),
    ("Ua", "RU'R URUR U'R'U'R2"),
];

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

    // M -> [str]
    let mut oll_tbl = HashMap::new();
    for s in oll {
        let mat = parse(&s);
        oll_tbl.entry(mat).or_insert(vec![]).push(s);
    }

    // M -> Id
    let mut id = 0;
    let mut h2i = HashMap::new();
    for k in oll_tbl.keys() {
        id += 1;
        h2i.insert(k, id);
    }
    dbg!(&oll_tbl);

    // Perm -> [(Id, Id)]
    let mut perm_comb = HashMap::new();
    for (perm_name, perm_seq) in PERMS {
        // Find sequence [A,B] to effect M.
        // BA = M
        // B = M(A^1)
        let mut ab_pairs = vec![];
        let m = parse(perm_seq);
        for a in oll_tbl.keys() {
            let b = m * a.inv();
            if oll_tbl.contains_key(&b) {
                ab_pairs.push((h2i.get(&a).unwrap(), h2i.get(&b).unwrap()));
            }
        }

        perm_comb.insert(perm_name, ab_pairs);
    }

    // Id -> Int
    let mut occurence = HashMap::new();
    for (_, list) in &perm_comb {
        for (a, b) in list {
            *occurence.entry(h2i.get(&a).unwrap()).or_insert(0) += 1;
            *occurence.entry(h2i.get(&b).unwrap()).or_insert(0) += 1;
        }
    }

    // [(Id, [str])]
    // Highest occurrence first.
    let mut classes = vec![];
    for (m, list) in oll_tbl {
        classes.push((h2i.get(&m).unwrap(), list));
    }
    classes.sort_by_key(|x| occurence.get(&x.0).unwrap());
    classes.reverse();
}
