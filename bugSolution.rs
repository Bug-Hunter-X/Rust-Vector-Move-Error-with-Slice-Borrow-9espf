fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let slice = &vec[..];
    // This is okay, because the slice only borrows data from the vector.
    println!("Slice: {:?}", slice);

    // Solution: Borrow the vector instead of moving it
    let new_vec = &vec.clone();
    println!("Original Vec: {:?}", vec);
    println!("Cloned Vec: {:?}", new_vec);
} 