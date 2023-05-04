#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (Vec<(String, i32)>,)| {
    let mut tree = path_tree::PathTree::new();
    let mut test_data = None;

    for (path, num) in &data.0 {
        test_data = Some((path,tree.insert(path, num)));
    }

    match test_data {
        Some(x) => {
            let url = tree.url_for(x.1, &[]);
            assert!(url.is_some());
        }
        None => {}
    }
});
