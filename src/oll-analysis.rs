use lib::*;
use rubikmaster::matrix::PermutationMatrix;

use clap::Clap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

#[derive(Clap, Debug)]
#[clap(name = "oll-comb")]
struct Opts {
    #[clap(name = "FILE")]
    file: String,
}
fn parse(s: &str) -> PermutationMatrix {
    let mut m  = PermutationMatrix::identity();
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

    // M -> [Seq]
    let mut oll_tbl = HashMap::new();
    oll_tbl.insert(PermutationMatrix::identity(), vec!["".to_owned()]);
    for s in oll {
        let mat = parse(&s);
        oll_tbl.entry(mat).or_insert(vec![]).push(s);
    }

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
        // Find sequence [A,B] to effect M.
        // BA = M
        // B = M(A^1)
        let mut ab_pairs = vec![];
        let m = parse(perm_seq);
        for a in oll_tbl.keys() {
            let b = m * a.inv();
            if oll_tbl.contains_key(&b) {
                ab_pairs.push((*h2i.get(&a).unwrap(), *h2i.get(&b).unwrap()));
            }
        }

        perm_comb.insert(perm_name.to_owned(), ab_pairs);
    }
    let mut good_perms = vec![];
    for (perm_name, perm_seq) in &perm_comb {
        if perm_seq.len() > 0 {
            good_perms.push(perm_name.clone())
        }
    }

    // Id -> Int
    let mut occurences: HashMap<Id, u64> = HashMap::new();
    for (_, list) in &perm_comb {
        for (a, b) in list {
            *occurences.entry(*a).or_insert(0) += 1;
            *occurences.entry(*b).or_insert(0) += 1;
        }
    }

    let mut m = 0;
    for &n in occurences.values() {
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
    classes.sort_by_key(|x| occurences.get(&x.0).unwrap_or(&0));
    classes.reverse();

    let out = Analysis {
        n: classes.len(),
        m,
        c: good_perms.len(),
        good_perms,
        classes,
        occurences,
        perms: perm_comb,
    };
    let mut file = File::create("analysis.json").unwrap();
    let out = serde_json::to_string(&out).unwrap();
    write!(file, "{}", out).unwrap();
    file.flush().unwrap();
}