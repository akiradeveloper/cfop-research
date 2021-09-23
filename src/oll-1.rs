use rubikmaster::cfop;
use rubikmaster::matrix::{of, PermutationMatrix};
use rubikmaster::{Command, Move::*};
use std::collections::HashMap;

const NOTE_TBL: [&str; 15] = [
    "", "R", "R'", "R2", "F", "F'", "U", "U'", "U2", "L", "L'", "D", "D'", "M'", "M2",
];
fn com(m: rubikmaster::Move, rep: i8) -> PermutationMatrix {
    of(Command(m, rep))
}
fn main() {
    let mut mov_tbl = [PermutationMatrix::identity(); 15];
    mov_tbl[0] = PermutationMatrix::identity();
    mov_tbl[1] = com(R, 1);
    mov_tbl[2] = com(R, -1);
    mov_tbl[3] = com(R, 2);
    mov_tbl[4] = com(F, 1);
    mov_tbl[5] = com(F, -1);
    mov_tbl[6] = com(U, 1);
    mov_tbl[7] = com(U, -1);
    mov_tbl[8] = com(U, 2);
    mov_tbl[9] = com(L, 1);
    mov_tbl[10] = com(L, -1);
    mov_tbl[11] = com(D, 1);
    mov_tbl[12] = com(D, -1);
    mov_tbl[13] = com(M, -1);
    mov_tbl[14] = com(M, 2);

    let mut ans = vec![];
    let mut done = 0;
    let n = 15;
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
}
