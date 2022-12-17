use std::collections::HashMap;

pub struct Context {
    symbols: HashMap<String, (u64 /* address */, u64 /* length */)>,
    functions: Vec<u64>,
}
