extern crate sled;
fn main() {}

fn hello() -> Result<(), sled::Error> {
    let tree = sled::open("/tmp/sledding-attempt")?;

    let k = "KEY";
    let v1 = "VAL1";
    let v2 = "VAL2";

    // insert and get, similar to std's BTreeMap
    tree.insert(k, v1).expect("inserted");
    let found = tree.get(&k);

    // range queries
    // TODO for kv in tree.range(k..100) {}

    // deletion
    tree.remove(&k).expect("removed");

    // atomic compare and swap
    tree.compare_and_swap(k, Some(v1), Some(v2))
        .expect("swapped")
        .expect("noreally");

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)
    tree.flush().expect("whoooosh");

    Ok(())
}
