mod file_read;
mod syntax_map;

use crate::syntax_map::MemStore;
fn main() {
    let mut store = syntax_map::MemStore::new();
    MemStore::initialize(&mut store);
    let file_path = "../docs/ma_parser.md";
    let data = file_read::read_file(file_path).unwrap();
    println!("file data:{:?}", data.to_vec());
}
