mod syntax_map;
use crate::syntax_map::MemStore;
fn main() {
    let mut store = syntax_map::MemStore::new();
    MemStore::initialize(&mut store);
    if MemStore::find_by_key(&store, "# ") {
        println!("key match found");
    } else {
        println!("no matching key found");
    }
}
