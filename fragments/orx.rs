use orx_parallel::prelude::*;
use std::path::PathBuf;

struct Grammar {
    rules_count: usize,
    _marker: std::marker::PhantomData<std::rc::Rc<()>>, // Forces non-Sync status
}

struct SendSyncWrapper<T>(T);
unsafe impl<T> Sync for SendSyncWrapper<T> {}

fn main() {
    let inputs = vec![
        PathBuf::from("src/main.rs"),
        PathBuf::from("src/parser.rs"),
        PathBuf::from("src/grammar.rs"),
        PathBuf::from("src/lexer.rs"),
    ];

    let heavy_grammar = SendSyncWrapper(Grammar {
        rules_count: 42,
        _marker: std::marker::PhantomData,
    });

    // map_per_thread executes its closure EXACTLY ONCE per spawned thread core.
    // It passes a private, lock-free sequential iterator containing that thread's share of work.
    let thread_outputs: Vec<String> = inputs
        .par()
        .map_per_thread(|local_file_iter| {
            // 1. Initialize EXACTLY ONE string per thread
            let mut thread_log = String::new();
            let grammar = &heavy_grammar.0;

            // 2. This thread pulls files directly from its local iterator
            for file in local_file_iter {
                thread_log.push_str(&format!("Parsed {}; ", file.display()));
            }

            // 3. Return exactly ONE string for this entire thread
            thread_log
        })
        .collect(); // Collects a flat Vec<String>, one element per thread core!

    // The length of thread_outputs is exactly your CPU thread pool count (e.g., 2, 4, 8)
    println!("Final Thread Outputs: {:#?}", thread_outputs);
}
