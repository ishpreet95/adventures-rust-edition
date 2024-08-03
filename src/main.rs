fn main() {
    // first lines of code
    println!("Hello, world!");
    println!("Will this come in a new line?");

    // testing vector
    let x: Vec<i32> = vec![1, 2, 3, 4, 5];
    for i in &x {
        println!("{}", i)
    }

    // testing floating point precision
    let q = 5.777777777777772;
    let w = 0.000000000000001;
    println!("{}", q + w);

    // testing indexing
    println!("{}", x[1]);

    // testing iterator
    println!("Testing iterator");
    for &num_j in x.iter() {
        println!("{}", num_j);
    }

    println!("Testing iterator with enumerate");
    for (j, &num_j) in x.iter().enumerate() {
        println!("{} {}", j, num_j);
    }
}
