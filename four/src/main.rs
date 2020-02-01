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
    let accum1 = VecAccumulator { items: Vec::new() };
    let accum1_sum = takes_ownership_sum(accum1);
    println!("{}", accum1_sum);

    let accum1_sum2 = takes_ownership_sum(accum1); // oops

    let accum2 = VecAccumulator { items: Vec::new() };
    let (accum3, accum2_sum) = takes_and_gives_ownership_sum(accum2);
    println!("{}", accum2_sum);
    let accum2_sum2 = takes_ownership_sum(accum2); // oops
    let accum3_sum = takes_ownership_sum(accum3); // not an oops
    println!("{}", accum3_sum);

    let inter = 5;
    copy_not_move(inter);
    println!("{}", inter); // not an oops


    let a = VecAccumulator { items: Vec::new() };
    drop(a);
    println!("{:?}", a); // opps
}
