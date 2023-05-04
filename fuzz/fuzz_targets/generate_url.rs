#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (Vec<(String, i32)>,)| {
    let mut tree = path_tree::PathTree::new();
    let mut id = None;

    for (path, num) in &data.0 {
        id = Some(tree.insert(path, num));
    }

    match id {
        Some(x) => {
            tree.url_for(x, &[]);
        }
        None => {}
    }
});
