fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    print!("{len}\n");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem since r1 & r2 are not used at this point
    // if I tried to println r1 or r2 here it wouldn't compile since
    // rust prevents possible data races from mutable and immutable
    // references to variables
    println!("{r3}");

}

// Takes a reference to s which basically means it passed the memory address and "borrows" it.
// Doing this avoids transfer of ownership so the `calculate_length` function never takes
// ownership of s.
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.