use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::slice;

fn main() {
    unsafe_rust();
    advanced_traits();
    advanced_types();
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

    let mut x = vec![1, 2, 3, 4, 5];

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

fn advanced_traits() {
    //There are things called associated types. These are similar to generics except that with
    // associated types the type can only be implemented once. This allows for the type to not
    // need to be explicitly specified each time.

    trait FooAssociated {
        type Item;

        fn foo_associated(&mut self) -> Option<Self::Item>;
    }

    trait FooGeneric<T> {
        fn foo_generic(&mut self) -> Option<T>;
    }

    struct BarStruct;

    impl FooAssociated for BarStruct {
        type Item = u32;

        fn foo_associated(&mut self) -> Option<Self::Item> {
            Some(3)
        }
    }

    //This code will not compile because only a single implementation can exist for an associated
    // type.
    // impl FooAssociated for BarStruct {
    //     type Item = String;
    //
    //     fn foo_associated(&mut self) -> Option<Self::Item> {
    //         Some(String::from("associated"))
    //     }
    // }

    impl FooGeneric<u32> for BarStruct {
        fn foo_generic(&mut self) -> Option<u32> {
            Some(5)
        }
    }

    impl FooGeneric<String> for BarStruct {
        fn foo_generic(&mut self) -> Option<String> {
            Some(String::from("generic"))
        }
    }

    let mut bar = BarStruct {};

    //Note that the generics are more complex to call. However, if there is only a single
    // implementation this is not true in this case. It seems to be that there are some benefits
    // to be had with the compiler and that it can make more guarantees here. The way of calling
    // the different generics is know as `fully qualified syntax`. It is explored a little bit more
    // below
    println!(
        "associated {:?} generic::u32 {:?} generic::String {:?}",
        bar.foo_associated(),
        <BarStruct as FooGeneric<u32>>::foo_generic(&mut bar),
        <BarStruct as FooGeneric<String>>::foo_generic(&mut bar),
    );

    //A default type can be set for a parameter.
    trait Winner<T=u32> {
        type Output;

        fn win(self, num: T) -> T;
    }

    struct Check;

    //Notice that a type does not need to be explicitly specified here. Instead, the default type
    // is used.
    impl Winner for Check {
        type Output = ();

        fn win(self, num: u32) -> u32 {
            num
        }
    }

    let check = Check{};

    println!("win {}", check.win(4));

    //Fully qualified syntax can be used when there are conflicting names.

    trait Arm {
        fn pain(&self) {
            println!("My arm feels good");
        }
    }

    trait Leg {
        fn pain(&self) {
            println!("My leg is a little sore");
        }
    }

    struct Human;

    impl Arm for Human {}

    impl Leg for Human {}

    impl Human {
        fn pain(&self) {
            println!("Overall I feel good");
        }
    }

    let human = Human{};

    //The below is fully qualified syntax. By default the Human implementation of pain() is called.
    // However, if other implementations of pain() are needed, they can also be called using the
    // below syntax.
    human.pain();
    Arm::pain(&human);
    Leg::pain(&human);

    //Note that fully qualified syntax can be used anywhere. However, Rust can figure out most of
    // it and so there is no need.
    <Human as Leg>::pain(&human);

    //Supertraits are traits that are required to implement another trait.
    trait ShowStuff: Display {
        fn show_stuff(&self) {
            println!("running show_stuff() {}", self.to_string());
        }
    }

    struct Box{len: i32}

    //Display must be implemented in order to implement the trait ShowStuff.
    impl Display for Box {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.len)
        }
    }

    impl ShowStuff for Box {}

    let my_box = Box{len: 12};

    my_box.show_stuff();

    //There is also something called the `newtype pattern`. The terminology is apparently taken
    // from Haskell. Essentially there is a rule that restricts from implementing an external trait
    // on an external type. In order to get around this, a wrapper can be made for the external
    // type and the trait can be implemented on the wrapped. The example given in the book for this
    // is listed below. Apparently the compiler will use elision to remove any performance penalty
    // when this pattern is used.
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

}

fn advanced_types() {
    //The newtype pattern can also be used to hide implementation details. For example a HashMap
    // could have a Wrapper that makes the API for it more conceptual.

    //There is something similar to typedef in C++ that can be done to alias types. It seems to
    // mostly be used to shorten long type names.
    type Hi = HashMap<Vec<i32>, HashMap<Vec<String>, HashSet<u32>>>;

    let hello: Hi = HashMap::new();

    println!("hello: {:?}", hello);

    //As a fun note, the reason the namespace can be eliminated is because of type aliasing inside
    // the standard library. For example, type HashMap<T, U> = std::HashMap<T, U>.

    //There is a `never type` that is returned as shown below in foo. This means that a type is
    // never returned from this. So for example things link `continue` and `panic!` return this
    // type. This allows for types to be properly be determined inside things like match statements.
    fn foo() -> ! {
        panic!("never type");
    }

    if false { foo(); }

    //Essentially rust stores both the memory address as well as the size of the memory when
    // handling dynamic memory. This seems to be done for support for slicing. For example, an &str
    // type does this because the size is unknown until compile time. This is a bit different than
    // in C or C++, in those languages a reference is simply a pointer. Then either the size is
    // stored internally to the object or passed separately. Either way it must be handled manually.

    //By nature, a generic type has the follow type automatically put on it by the compiler.
    fn _generic<T: Sized>(_t: T) {
        // --snip--
    }

    //This type can be overridden using the following method.
    fn _generic_unsized<T: ?Sized>(_t: &T) {
        // --snip--
    }
}
