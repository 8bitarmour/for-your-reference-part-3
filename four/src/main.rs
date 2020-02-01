// What about fields of structs? The binding is "owned" by the struct
// so here items will be dropped whenever the owning VecAccumulator is
// dropped.
#[derive(Debug)]
struct VecAccumulator {
    items: Vec<i64>,
}

fn takes_ownership_sum(owned_accum: VecAccumulator) -> i64 {
    let mut sum = 0;
    for item in owned_accum.items.iter() {
        sum += item;
    }
    sum
} // owned_accum goes out of scope and we are the owner, so rust drops it

fn takes_and_gives_ownership_sum(owned_accum: VecAccumulator) -> (VecAccumulator, i64) {
    let mut sum = 0;
    for item in owned_accum.items.iter() {
        sum += item;
    }
    (owned_accum, sum)
} // owned_accum goes out of scope but we are "moving" it back to callling scope, so we don't call drop

fn copy_not_move(my_int: i64) {
    println!("\nPrinting copied value {}.", my_int);
}

fn main() {
    // How does this fit in with function calls?
    // In the same way that we can move ownership from one let binding to another
    // we can also move ownership to parameter bindings:
    let accum1 = VecAccumulator { items: Vec::new() };
    let accum1_sum = takes_ownership_sum(accum1);
    println!("{}", accum1_sum);

    // Cannot sue accum binding here since it has been invalidated/moved into the takes_ownership function
    // We cannot use accum1 since takes_ownership has dropped it!
    let accum1_sum2 = takes_ownership_sum(accum1); // oops

    // Well what happens when we just want to compute the sum but still keep the ownership of
    // the accumulator because we're not done with it?
    let accum2 = VecAccumulator { items: Vec::new() };
    // Here we give the ownership to the function but then ask it to give it right back
    // now accum3 is the owner, and accum2 binding has been invalidated.
    let (accum3, accum2_sum) = takes_and_gives_ownership_sum(accum2);
    println!("{}", accum2_sum);
    let accum2_sum2 = takes_ownership_sum(accum2); // oops
    let accum3_sum = takes_ownership_sum(accum3); // not an oops
    println!("{}", accum3_sum);

    // Jeeze, does this mean we have to do this give and take pattern every time we want to
    // "loan" out data to process it in some way. That's annoying and unbearable in large programs
    // To combat this, rust has references. We'll cover that in the next episode.

    // Final note, just like let bindings, functions will also copy
    let inter = 5;
    copy_not_move(inter);
    println!("{}", inter); // not an oops

    // Bonus! Moving ownerhship is how the drop() function works
    // https://doc.rust-lang.org/src/core/mem/mod.rs.html#756
    // Signature _moves_ the T into the drop funciton so we can no longer use it
    let a = VecAccumulator { items: Vec::new() };
    drop(a);
    println!("{:?}", a); // opps
}
