fn main() {
    unsafe_rust();
}

fn unsafe_rust() {
    //By nature it is better for Rust to reject some valid programs than to allow some invalid
    // programs. So unsafe Rust is used to allow the user to override when they know better than
    // the compiler. Also there are certain things such as communicating directly with the OS that
    // Rust cannot do without unsafe mode.

    //There are five actions that can be taken in unsafe Rust.
    // 1) Dereference raw pointers.
    // 2) Call an unsafe function or method.
    // 3) Access or modify a mutable static variable.
    // 4) Implement an unsafe trait.
    // 5) Access fields of unions.
    //Rust still has all checking done normally outside of these five features.

    //A raw pointer can be created without unsafe code.
    let mut x = 5;

    //An immutable pointer.
    let immutable_ptr = &x as *const i32;
    //A mutable pointer.
    let mutable_ptr = &mut x as *mut i32;

    //This will create a pointer to an arbitrary memory location.
    let address = 0x012345usize;
    let _unknown_if_valid = address as *const i32;

    unsafe {
        println!("mut_ptr: {}", *mutable_ptr);
        println!("ptr: {}", *immutable_ptr);

        //This can and probably will segfault if there is not valid memory at the location.
        // println!("unknown_ptr: {}", *unknown_if_valid);
    }
}

