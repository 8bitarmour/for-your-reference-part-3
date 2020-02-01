fn main() {

    println!("\nMoving:");
    {
        let vec1 = vec![1,2,3];
        println!("{:?}", vec1);
        let vec2 = vec1;
        println!("{:?}", vec2);

    }

    println!("\nUsing after move:");
    {
        let vec1 = vec![1,2,3];
        println!("{:?}", vec1);
        let vec2 = vec1;
        println!("{:?}", vec2);
        println!("{:?}", vec1); // opps
    }

    println!("\nCopying:");
    {
        let num1 = 3;
        let num2 = num1;
        println!("{}", num1);
        println!("{}", num2);
    }

    println!("\nCloning:");
    {
        let vec1 = vec![1,2,3];
        println!("{:?}", vec1);
        let mut vec2 = vec1.clone();

        vec2.push(4);
        println!("{:?}", vec2);
        println!("{:?}", vec1); // no longer an opps
    } // Rust drops both vec1 and vec2 here
}
