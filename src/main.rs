fn main() {
    //For Loop
    for x in 0..10 {
        println!("x = {}", x);
    }

    //while Loop
    let mut y = 0;
    while y < 10 {
        println!("y = {}", y);
        y += 1;
    }

    //Do while Loop
    let mut z = 0;
    loop {
        println!("z = {}", z);
        z += 1;
        if z == 10 {
            break;
        }
    }

    //For each Loop
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("i = {}", i);
    }

    
}
