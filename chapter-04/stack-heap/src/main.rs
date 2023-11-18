fn main() {
    // String literal is immutable and is known at compile time,
    // so it can copied from one variable to another
    let x: &str = "test";
    let y: &str = x;
    println!("{}", x);
    println!("{}", y);

    // String type IS mutable, and thus does not implement copy,
    // When y is assigned to x, a copy of the pointer on the stack
    // is made, NOT the string data allocated on the heap
    //
    // However! Rust does not have GC, so doing this is inherently
    // risky. When drop() is called on the String, Rust would attempt
    // to free the same memory twice! Once for each x and y.
    // Rust protects us at compile-time by detecting that x's 
    // referenced data has been moved to y. Thus it is a "move",
    // "NOT" a shallow copy
    let x: String = String::from("test");
    let y: String = x;
    // println!("{}", x);
    println!("{}", y);

    // Another place where copy vs. move becomes relevant is function
    // invocations. For data types that do not implement copy, the reference
    // is "moved" from the owner to the argument of the function. As with
    // the above, once the ownership is moved, the previous owner is invalidated,
    // and it is a compile-time error to reference the previous owner
    let z = String::from("method");
    method_pass(z);
    // println!("{}", z);

}

fn method_pass(str: String) -> String {
  return str;
}

// Returns also exhibit ownership move. When use_val is invoked,
// the ownership of the data is transferred from input variable
// to the argument. Returned values have ownership transferred from
// the function scope to the variable bound to the return value. In
// order to provide a value to a function call, you COULD return
// a tuple and use destructuring to shadow the variable, even if
// immutable (see str and mutable) below
fn test_returns() {
  let str: String = String::from("test");
  let mut mutable: String = String::from("mutable");
  let (mutable, str2) = use_val(mutable);
  let (str, str3) = use_val(str);
  println!("{}", mutable);
  println!("{}", str2);
  println!("{}", str3);
  println!("{}", str);
}

fn use_val(str: String) -> (String, String) {
  let str2: String = String::from("hello");
  return (str, str2);
}
