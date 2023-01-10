fn main() {
    let tup1 = (500, 6.4, 1);
    let tup2 = (19, 09, 96, ("sept", "fri"));
    let (x, y, z) = tup1;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    println!("The value of tup1 is: {}", tup1.2);
    println!("The value of tup2 is: {}", (tup2.3).0);



    
}
