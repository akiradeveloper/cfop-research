use std::collections::HashMap;

pub type Id = u64;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Analysis {
    pub n: usize,
    pub m: usize,
    pub occurences: HashMap<Id, u64>,
    pub perms: HashMap<String, Vec<(Id, Id)>>,
    pub classes: Vec<(Id, Vec<String>)>,
}
