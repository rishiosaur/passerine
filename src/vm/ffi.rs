use std::{collections::HashMap, fmt::Debug};
use crate::common::data::Data;

use super::trace::Trace;


pub struct FFI {
    pub bindings: HashMap<String, Box<dyn Fn(Data) -> Result<Data, Trace>>>
}

impl Debug for FFI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot debug FFi (...)")
    }
}

