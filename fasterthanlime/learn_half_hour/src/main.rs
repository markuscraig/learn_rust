fn main() {
    variable_bindings();
    tuples();
}

fn variable_bindings() {
    // let keyword with type inferred
    let x;
    x = 42;
    println!("x = {x}");

    // let keyword as single line
    let y = 42;
    println!("y = {y}");

    // with type annotation
    let x2: i32;
    x2 = 43;
    println!("x2 = {x2}");

    // uninitialized variable gives the
    // 'type annotations needed' compile error
    //let bad;
    //println!("bad = {bad}");

    // use _ to throw away values
    let _ = 42;  // does nothing
    let _ = get_val();

    // _ prefix causes compiler to ignore unused error
    let _not_used = 42;

    // shadow bindings reuses the previous name
    let x3 = 42;
    let x3 = 84;
    println!("x3 = {x3}");
}

fn get_val() -> i32 {
    return 42;
}

fn tuples() {
    // tuples are fixed length collections of
    // different types
    let pair = ('a', 17);
    println!("tuple pair: 0 = {}, 1 = {}", pair.0, pair.1);

    // tuple with type annotations
    let pair2: (char, i32) = ('b', 18);
    println!("tuple pair2: 0 = {}, 1 = {}", pair2.0, pair2.1);

    // destructing tuples into fields
    let (my_char, my_int) = pair2;
    println!("my_char = {}, my_int = {}", my_char, my_int);

    // can throw away part of a destructed tuple
    let (_, right) = pair2;
    println!("right = {}", right);
}
