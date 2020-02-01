#[derive(Debug)]
struct Accumulator {
    sum: i64,
}

fn main() {
    let accum = Accumulator { sum: 0 };
    println!("{:?}\n", accum);

    let dynamic_array = vec![1, 2, 3];
    println!("{:?}\n", dynamic_array);

    let open_file = std::fs::File::open("data/test.json").unwrap();
    println!("file length is {}", open_file.metadata().unwrap().len());

    drop(open_file);

    // Rust will insert drop(dynamic_array) here for us
    // This is because the binding dynamic_array is the _owner_ of the data/resource and is
    // going out of scope here at the end curly
}

// Here are the rules for ownership:
//  - Every value has an owner in rust (so we can track when to free)
//  - There can be only one owner of a value at a time (so that we don't double free)
//  - When the owner goes out of scope, the value is dropped (so we can free exactly once at that time)
