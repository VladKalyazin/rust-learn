use std::sync::Mutex;

pub struct Table {
    pub forks: Vec<Mutex<()>>,
}