// SPDX-License-Identifier: MIT
extern crate env_logger;
extern crate log;
extern crate sled;

use log::info;

fn main() {
    env_logger::init();
    hello().expect("Greetings!");
}

fn hello() -> Result<(), sled::Error> {
    let tree = sled::open("/tmp/sledding-attempt")?;

    let k = "KEY0";
    let v1 = "VAL1";
    let v2 = "VAL2";

    // insert and get, similar to std's BTreeMap
    tree.insert(k, v1).expect("inserted");
    let found = tree.get(&k).expect("found");
    info!("We found a value! {:?}", found);

    // range queries
    for kv in tree.range(k.."KEY9") {
        info!("Range query found: ");
        if let Ok((rk, rv)) = kv {
            info!("\t${:?} ${:?}", rk, rv);
        }
    }

    // deletion
    tree.remove(&k).expect("removed");

    tree.insert(k, v1).expect("it's back now");
    // atomic compare and swap
    tree.compare_and_swap(k, Some(v1), Some(v2))
        .expect("compared")
        .expect("swapped");

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)
    tree.flush().expect("whoooosh");

    Ok(())
}
