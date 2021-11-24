use std::collections::HashMap;
use std::hash::Hash;
use crate::engine::data::id_map::IdMap;
use crate::engine::data::type_map::TypeMap;

pub trait Word {

}

pub enum Noun {
    None,
    Singular(i32),
    Plural(Vec<i32>)
}

pub trait Relation: State {

}

pub trait State: Word {

}

pub struct Entity {
    is: TypeMap,
    has: IdMap<i32>,
}

pub struct World {
    entities: IdMap<i32>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: IdMap::new(),
        }
    }
}