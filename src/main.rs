mod file_read;
mod syntax_map;

use std::path::Path;

use crate::syntax_map::MemStore;
fn main() {
    let mut store = syntax_map::MemStore::new();
    MemStore::initialize(&mut store);
    let file_path = Path::new("./docs/ma_parser.md").to_str().unwrap();
    // TODO: usedata to build a lexical pair on each word
    let _data = file_read::read_file(file_path).unwrap();
}
