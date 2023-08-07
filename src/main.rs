use std::slice;

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

    //An unsafe function (which is essentially a block of unsafe code) must also be called inside
    // an unsafe block.
    unsafe fn danger() {
        println!("Dangerous function called!");
    }

    unsafe {
        danger();
    }

    //It is also possible to create a safe abstraction around the unsafe code.
    fn danger_two() {
        let mut x = 1;
        let ptr = &mut x as *mut i32;
        unsafe {
            println!("Another dangerous function {:?}", *ptr);
        }
    }

    danger_two();

    //As a side note, in order to get around the rules of the Rust borrow checker generally raw
    // pointers are used. The rules that Rust uses only seem to apply to its smart pointers. So the
    // below code will work even though there are two different mutable references to the same
    // variable.
    unsafe fn hello(x: &mut Vec::<i32>) -> (&mut [i32], &mut [i32]) {
        let ptr = x.as_mut_ptr();

        //These variable are mutable, the pointers themselves are not mutable.
        let first = slice::from_raw_parts_mut(ptr, 3);
        let second = slice::from_raw_parts_mut(ptr.add(3), 2);

        (first, second)
    }

    let mut x = vec![1,2,3,4,5];

    unsafe {
        println!("Unsafe stuff: {:?}", hello(&mut x));
    }

    //Different language functions can be called from inside Rust. Below calls the abs() function
    // from the `C` programming language. These calls must always be done inside unsafe blocks.
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    //Rust supports static variables as well. The difference between const variables and static
    // variables is that static variables have a fixed memory address. The static mut variable
    // cannot changed unless unsafe code is used.
    static mut COUNTER: isize = 0;

    //Calls to access the static mut variable must be done inside the unsafe block.
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {COUNTER}");
    }


    //They don't go into much detail, but an unsafe trait can be used as well.
    unsafe trait Foo {}

    //Unions are the final way that unsafe code works. unions are apparently like structs. However,
    // their primary use is to interface with `C` language unions.
}


