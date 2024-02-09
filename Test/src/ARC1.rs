

//https://henrybarreto.dev/box-rc-arc-and-the-relation-with-threads
//https://www.sitepoint.com/rust-global-variables/
//https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/std/thread_local/index.html#:~:text=Module%20std%3A%3Athread_localStable&text=This%20module%20provides%20an%20implementation,not%20need%20to%20be%20synchronized.

use std::thread;
use std::sync::Arc;
enum Blockchain {
    Block(u32, Arc<Blockchain>),
    End
}

use Blockchain::{Block, End};

fn main() {
    let blockchain = Arc::new(
        Block(0, 
            Arc::new(Block(1,
            Arc::new(Block(2,
            Arc::new(End)))))
    ));

    for t in 3..10 {
        let clone = blockchain.clone();
        thread::spawn(move || {
            let _blockchain_thread = Block(t, clone);
        }).join().unwrap();
    }
}


