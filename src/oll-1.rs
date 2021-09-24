use rubikmaster::cfop;
use rubikmaster::matrix::{of, PermutationMatrix};
use rubikmaster::{Command, Move::*};

use clap::Clap;
use std::fs::File;
use std::io::Write;

#[derive(Clap, Debug)]
#[clap(name = "oll-1")]
struct Opts {
    #[clap(short)]
    n: usize,
}

const NOTE_TBL: [&str; 15] = [
    "", "R", "R'", "R2", "U", "U'", "U2", "F", "F'", "L", "L'", "D", "D'", "M'", "M2",
];
fn cmd(m: rubikmaster::Move, rep: i8) -> PermutationMatrix {
    of(Command(m, rep))
}
fn main() {
    let opt = Opts::parse();

    let mut mov_tbl = [PermutationMatrix::identity(); 15];
    mov_tbl[0] = PermutationMatrix::identity();
    mov_tbl[1] = cmd(R, 1);
    mov_tbl[2] = cmd(R, -1);
    mov_tbl[3] = cmd(R, 2);
    mov_tbl[4] = cmd(U, 1);
    mov_tbl[5] = cmd(U, -1);
    mov_tbl[6] = cmd(U, 2);
    mov_tbl[7] = cmd(F, 1);
    mov_tbl[8] = cmd(F, -1);
    mov_tbl[9] = cmd(L, 1);
    mov_tbl[10] = cmd(L, -1);
    mov_tbl[11] = cmd(D, 1);
    mov_tbl[12] = cmd(D, -1);
    mov_tbl[13] = cmd(M, -1);
    mov_tbl[14] = cmd(M, 2);

    let mut ans = vec![];
    let mut done = 0;
    let n = opt.n;
    for (x0, x1, x2, x3, x4, x5, x6, x7, x8) in
        itertools::iproduct!(0..n, 0..n, 0..n, 0..n, 0..n, 0..n, 0..n, 0..n, 0..n)
    {
        let mut m = PermutationMatrix::identity();
        let mut ap = |i| {
            let op = mov_tbl[i];
            m = op * m;
        };
        ap(x0);
        ap(x1);
        ap(x2);
        ap(x3);
        ap(x4);
        ap(x5);
        ap(x6);
        ap(x7);
        ap(x8);
        let ok = cfop::f2l_solved(&m) && !cfop::solved(&m);
        if ok {
            let mut s = String::new();
            s.push_str(NOTE_TBL[x0]);
            s.push_str(NOTE_TBL[x1]);
            s.push_str(NOTE_TBL[x2]);
            s.push_str(NOTE_TBL[x3]);
            s.push_str(NOTE_TBL[x4]);
            s.push_str(NOTE_TBL[x5]);
            s.push_str(NOTE_TBL[x6]);
            s.push_str(NOTE_TBL[x7]);
            s.push_str(NOTE_TBL[x8]);
            ans.push(s);
        }

        done += 1;
        if done % 100000 == 0 {
            println!("done: {}, found: {}", done, ans.len());
        }
    }

    let mut file = File::create("out.json").unwrap();
    let out = serde_json::to_string(&ans).unwrap();
    write!(file, "{}", out).unwrap();
    file.flush().unwrap();
}
