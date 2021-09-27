use std::collections::{HashSet, HashMap};

pub type Id = u64;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OLLEnumeration {
    pub rotations: Vec<String>,
    pub oll_n: usize,
    pub set: HashSet<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Analysis {
    pub n: usize,
    pub m: usize,
    pub c: usize,
    pub good_perms: HashMap<String, Vec<i8>>,
    pub occurrences: Vec<(Id, u64)>,
    pub perms: Vec<(String, i8, Vec<(Id, Id)>)>,
    pub classes: Vec<(Id, Vec<String>)>,
}
