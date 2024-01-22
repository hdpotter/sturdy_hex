use crate::*;
use std::collections;

pub trait HexData<T> {
    fn hex_in_domain(&self, hex: HexCoord) -> bool;
    
    fn get(&self, hex: HexCoord) -> Option<&T>;
    fn contains_hex(&self, hex: HexCoord) -> bool {
        self.get(hex).is_some()
    }

    fn insert(&mut self, hex: HexCoord, value: T) -> Option<T>;
    fn remove(&mut self, hex: HexCoord) -> Option<T>;
}

pub struct HashMapHexData<T> {
    data: collections::HashMap<HexCoord, T>,
}

impl<T> HashMapHexData<T> {
    pub fn new() -> HashMapHexData<T> {
        HashMapHexData {
            data: collections::HashMap::new(),
        }
    }
}

impl<T> HexData<T> for HashMapHexData<T> {
    fn hex_in_domain(&self, _: HexCoord) -> bool {
        true
    }

    fn get(&self, hex: HexCoord) -> Option<&T> {
        self.data.get(&hex)
    }

    fn insert(&mut self, hex: HexCoord, value: T) -> Option<T> {
        self.data.insert(hex, value)
    }

    fn remove(&mut self, hex: HexCoord) -> Option<T> {
        self.data.remove(&hex)
    }
}


