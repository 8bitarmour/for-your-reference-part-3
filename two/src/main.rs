// Here are the rules for ownership:
//  - Every value has an owner in rust (so we can track when to free)
//  - There can be only one owner of a value at a time (so that we don't double free)
//  - When the owner goes out of scope, the value is dropped (so we can free exactly once at that time)

#[derive(Debug)]
struct Accumulator {
    sum: i64,
}

fn main() {
    // Memory of a fixed known size can be allocated on the stack
    // This is made on the stack (main program frame):
    let accum = Accumulator { sum: 0 };
    println!("{:?}\n", accum);

    // But some allocations have additional "resources" associated with them
    // Here the vector has a length/capacity and pointer on the stack
    // but the actual data is a block of memory on the heap:
    let dynamic_array = vec![1, 2, 3];
    println!("{:?}\n", dynamic_array);

    // Here we have a File value here that has some info about the file
    // but there's also an open file descriptor that we want the OS
    // to close:
    let open_file = std::fs::File::open("data/test.json").unwrap();
    println!("file length is {}", open_file.metadata().unwrap().len());

    // We want to "free" the vector heap allocation and close the file
    // descriptor (and exactly once). If we forget, we leak. If we do it early
    // that's a safety issue, and if we do it more than once, that's also a bug.

    // One solution would be a garbage collector in the runtime.

    // In rust, there is no garbage collector, but the compiler does the same thing
    // at compile time.
    // The natural place for us to do this would be when open_file and
    // dynamic_array go out of scope (just like a GC book keeps on references)

    // In rust, the drop() function is how resources are de-allocated/destroyed
    // and we don't have to to write it by hand. The compiler puts these calls in
    // automatically for us when values go out of scope using "ownership"

    // We can still drop manually if we so choose to do early, closing the file
    drop(open_file);

    // Rust will insert drop(dynamic_array) here for us
    // This is because the binding dynamic_array is the _owner_ of the data/resource and is
    // going out of scope here at the end curly
}
