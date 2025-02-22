fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointers, use the vector's indexing capabilities.
    v[0] = 10;
    println!("The first element is: {}", v[0]);
}
