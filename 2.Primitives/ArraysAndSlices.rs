use std::mem;

// This function borrows a slice
fn analyze_slice(slice : &[i32]){
    println!("First Element of the Slice :{}", slice[0]);
    println!("The SLice has {} elements", slice.len());
}


fn main(){
    // Fixed-Size Array (Type Signature is superfluous).
    let xs:[i32;5] = [1,2,3,4,5];

    // All elements can be intialzied  to the same value.
    let ys:[i32;500] = [0;500];

    // Indexing stats at 0
    println!("First element of the array:{}", xs[0]);
    println!("Second element of the arrat:{}", xs[1]);


    // `len` returns the count of elements  in the array.
    println!("The number of elements in the array:{}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically  borrowed  as slices
    println!("Borrow the whole array  as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the  array as slice");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`
    let empty_array:[u32;0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}