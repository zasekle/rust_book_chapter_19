pub trait HelloMacro {
    //Note that Rust does not have reflection so it cannot look up the typename at runtime. This
    // means that for this situation, a default implementation cannot work. However, a macro can
    // work.
    fn hello_macro();
}
