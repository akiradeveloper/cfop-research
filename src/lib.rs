use std::collections::HashMap;

pub type Id = u64;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Analysis {
    pub n: usize,
    pub m: usize,
    pub c: usize,
    pub good_perms: Vec<(String, i8)>,
    pub occurences: HashMap<Id, u64>,
    pub perms: Vec<(String, i8, Vec<(Id, Id)>)>,
    pub classes: Vec<(Id, Vec<String>)>,
}
