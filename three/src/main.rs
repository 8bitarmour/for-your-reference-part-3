fn main() {

    println!("\nMoving:");
    {
        // We said that every value has exactly one owner, so whats going on here:
        let vec1 = vec![1,2,3];
        println!("{:?}", vec1);
        let vec2 = vec1;
        println!("{:?}", vec2);

        // We call this a "move". As in vec1 ownership was _moved_ to vec2; vec1 _was_ the owner
        // but then it gave it up to vec2.
        // Now at this end curly both vec2 and vec1 go out of scope, but since vec2 owns some data
        // at this time (vec1 does not anymore), rust inserts a drop(vec2) for us.
        // This also demonstrates another related concept in rust. Due to the rules of ownership
        // (namely that there can be only one owner at a time), we _invalidate_ references once
        // they are no longer the owner.
    }

    println!("\nUsing after move:");
    {
        // This is the same code but references vec1 _after_ the move and consequently doens't compile
        let vec1 = vec![1,2,3];
        println!("{:?}", vec1);
        let vec2 = vec1;
        println!("{:?}", vec2);
        println!("{:?}", vec1); // opps
    }

    println!("\nCopying:");
    {
        // One additional wrinkle: the following code works.
        let num1 = 3;
        let num2 = num1;
        println!("{}", num1);
        println!("{}", num2);
        // Wait, what? we used num1 after we moved to num2. why does this compile?
        // Well, small primatives (floats/ints, bools, chars and tuples of these)
        // implement the Copy trait and therefore are copied instead of moved so
        // here num1/num2 actually own different values in memory.
    }

    println!("\nCloning:");
    {
        // One way to make the vec example that doesn't compile above work is to explicity
        // copy the vec with teh `.clone()` method. Types that implement clone() can
        // explicity avoid moving by (deep) copying and therefore there are actually
        // two owners here of two seperate values here
        let vec1 = vec![1,2,3];
        println!("{:?}", vec1);
        let mut vec2 = vec1.clone();

        vec2.push(4);
        println!("{:?}", vec2);
        println!("{:?}", vec1); // no longer an opps
        // Clone is considered "expensive" while copy should be considered cheap by default
    } // Rust drops both vec1 and vec2 here
}
