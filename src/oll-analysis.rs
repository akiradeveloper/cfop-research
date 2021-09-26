use lib::*;
use rayon::iter::IntoParallelIterator;
use rubikmaster::cfop;
use rubikmaster::matrix::{of, PermutationMatrix};
use rubikmaster::{Command, Move};

use clap::Clap;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::time::Instant;
// use rayon::iter::FromParallelIterator;

#[derive(Clap, Debug)]
#[clap(name = "oll-comb")]
struct Opts {
    #[clap(name = "FILE")]
    file: String,
}
fn parse(s: &str) -> PermutationMatrix {
    let mut m = PermutationMatrix::identity();
    let es = rubikmaster::parser::parse(&s).unwrap().1;
    let cs = rubikmaster::flatten(es);
    for c in cs {
        m = rubikmaster::matrix::of(c) * m;
    }
    m
}
fn main() {
    let opt = Opts::parse();
    let file = File::open(opt.file).unwrap();
    let reader = BufReader::new(file);
    let oll: Vec<String> = serde_json::from_reader(reader).unwrap();
    dbg!(oll.len());

    // M -> [Seq]
    let mut oll_tbl = HashMap::new();
    oll_tbl.insert(PermutationMatrix::identity(), vec!["".to_owned()]);

    let s = Instant::now();
    let oll: Vec<_> = oll.into_par_iter().map(|s| (parse(&s), s)).collect();
    println!("{:?}", s.elapsed());

    let s = Instant::now();
    for (mat, s) in oll {
        oll_tbl.entry(mat).or_insert(vec![]).push(s);
    }
    println!("{:?}", s.elapsed());

    // M -> Id
    let mut id: u64 = 0;
    let mut h2i = HashMap::new();
    for &k in oll_tbl.keys() {
        id += 1;
        h2i.insert(k, id);
    }

    // Perm -> [(Id, Id)]
    let mut perm_comb = HashMap::new();
    for (perm_name, perm_seq) in rubikmaster::cfop::PLL_LIST {
        // Math:
        // M M' = Id
        // Mk (Mk'=y^k M') = Id
        // BA = Mk
        // B = Mk A'
        let m = parse(perm_seq);
        let m_inv = m.inv();
        for ny in 0..4 {
            let mut ab_pairs = vec![];
            let mk_inv = of(Command(Move::y, ny)) * m_inv;
            assert!(cfop::oll_solved(&mk_inv));
            let mk = mk_inv.inv();
            for a in oll_tbl.keys() {
                let b = mk * a.inv();
                if oll_tbl.contains_key(&b) {
                    ab_pairs.push((*h2i.get(&a).unwrap(), *h2i.get(&b).unwrap()));
                }
            }
            perm_comb.insert((perm_name.to_owned(), ny), ab_pairs);
        }
    }

    let mut good_perms = HashMap::new();
    for (k, v) in &perm_comb {
        if v.len() > 0 {
            good_perms.entry(k.0.to_owned()).or_insert(vec![]).push(k.1);
        }
    }

    // Id -> Int
    let mut occurrences: HashMap<Id, u64> = HashMap::new();
    for (_, list) in &perm_comb {
        for (a, b) in list {
            *occurrences.entry(*a).or_insert(0) += 1;
            *occurrences.entry(*b).or_insert(0) += 1;
        }
    }
    let mut m = 0;
    for &n in occurrences.values() {
        m += n as usize;
    }
    m /= 2;

    // [(Id, [Seq])]
    // Highest occurrence first.
    let mut classes = vec![];
    for (m, mut list) in oll_tbl {
        list.sort_by_key(|x| x.len());
        classes.push((*h2i.get(&m).unwrap(), list));
    }
    classes.sort_by_key(|x| occurrences.get(&x.0).unwrap_or(&0));
    classes.reverse();

    let mut occurrences: Vec<(Id, u64)> = occurrences.into_iter().collect();
    occurrences.sort_by_key(|x| x.1);
    occurrences.reverse();

    let out = Analysis {
        n: classes.len(),
        m,
        c: good_perms.len(),
        good_perms,
        classes,
        occurrences,
        perms: perm_comb.into_iter().map(|(k, v)| (k.0, k.1, v)).collect(),
    };
    let mut file = File::create("analysis.json").unwrap();
    let out = serde_json::to_string(&out).unwrap();
    write!(file, "{}", out).unwrap();
    file.flush().unwrap();
}
