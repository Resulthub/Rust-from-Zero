fn main() {
    let mut x = 19;

    {

        let dom= &mut x;
        *dom += 1;

        println!("x1 = {}", dom)
    }

    println!("x2 = {}", x);
}