fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let slice = &vec[..];
    // This is okay, because the slice only borrows data from the vector.
    println!("Slice: {:?}", slice);

    // This is where the error happens:
    let new_vec = vec.clone();
    // The vector is moved into 'new_vec', leading to an error if we still use 'vec'
    println!("Original Vec: {:?}", vec); 
}