fn main() {
    let mut my_vector = vec![1, 2, 3, 4, 5];

    my_vector.push(49);

    my_vector.remove(0);

    println!("{:?}", my_vector);

    for number in my_vector.iter(){
        println!("{}", number);
    }

}
