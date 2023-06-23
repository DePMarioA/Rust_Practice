fn main() {
    let x = vec!["string1".to_string()];
    let y = x;
    let z = y;
    println!("{:?}", z); // if we replace z with x or y error occurs because the values has been moved to z.

    //Deep vs Shallow copies
    //Deep is expensive
    let x = vec!["string1".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("from x: {:?}", x);
    println!("from y: {:?}", y);
    println!("from z: {:?}", z);

    // MOST TYPES IMPLEMENT A MOVE. AKA Strings..
    //SOME IMPLEMENT A COPY.
    //      example: integer,bool, char.
    //      tuples can be copied if items are one of the above.
    let x = 1;
    let y = x;
    println!("x={x}, y= {y}");
}
