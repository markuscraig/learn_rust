fn main() {
    variable_bindings();
    tuples();
    statements();
    functions();
    blocks();
    field_access();
    method_calling();
    modules();
    structs();
    patterns();
    methods();
    immutability();
    traits();
    generics();
    macros();
    panic_macro();
    funcs_that_panic();
    enums();
    lifetimes();
    static_lifetime();
    struct_literal();
    owned_vs_ref_types();
    fallible_functions();
    deferencing();
    function_types();
    loops();
    returning_closures();
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
    let _ = 42; // does nothing
    let _ = get_val();

    // _ prefix causes compiler to ignore unused error
    let _not_used = 42;

    // shadow bindings reuses the previous name
    let x3 = 42;
    let x3 = 84;
    println!("x3 = {x3}");
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

fn statements() {
    // semicolon marks end of a statement
    let x = 3;
    println!("x = {}", x);

    // statements can span multiple lines
    let y = vec![1, 2, 3]
        .iter()
        .map(|x| x + 3)
        .fold(0, |acc, x| acc + x);
    println!("y = {}", y);
}

fn functions() {
    // fn defines a function and the arrow
    // indicates the return type
    let x = get_val();
    println!("fn return x = {x}");
}

fn get_val() -> i32 {
    // last value is returned (without semicolon)
    //return 42;
    42 // the 'tail' which the block evaluates to
}

fn blocks() {
    // blocks are expressions which evaluate to a value
    let x = {
        42 // omitting the semicolon is an implicit return
    };
    println!("block x = {x}");

    // blocks can have multiple statements
    let x = {
        let y = 1;
        let z = 2;
        y + z // the 'tail' which the block evaluates to
    };
    println!("block multi x = {x}");

    // if conditions are expressions
    let enabled = true;
    let x = {
        if enabled {
            1
        } else {
            0
        }
    };
    println!("x enabled = {x}");
}

fn field_access() {
    // dots used to access fields
    let a = (10, 20);
    println!("a.0 = {}, a.1 = {}", a.0, a.1);
}

fn method_calling() {
    // dots used to call methods
    let name = "Mark";
    println!("name len = {}", name.len());
}

fn modules() {
    // use double-colon for module namespaces
    let least = std::cmp::min(3, 8);
    println!("least = {least}");

    // 'use' to import module symbols
    use std::cmp::min;
    let least = min(7, 1);
    println!("least with use = {least}");

    // different ways to import multiple symbols
    {
        use std::cmp::min;
        println!("min = {}", min(7, 1));
    }
    {
        use std::cmp::max;
        println!("max = {}", max(7, 1));
    }
    {
        use std::cmp::{max, min};
        println!("min2 = {}", min(7, 1));
        println!("max2 = {}", max(7, 1));
    }
    {
        use std::{cmp::max, cmp::min};
        println!("min3 = {}", min(7, 1));
        println!("max3 = {}", max(7, 1));
    }

    // wildcard import of all symbols
    use std::cmp::*;

    // types are namespaces and methods can
    // be called as regular functions
    let len = str::len("apples");
    println!("len = {len}");

    // the libstd prelude includes many non-primitive types
    let mut v = Vec::new();
    v.push(42);
    v.push(43);
    println!("vector size = {}", v.len());

    // Rust inserts this at the beginning of every module:
    //    "use std::prelude::v1::*"
    //
    // The prelude exports lots of symbols like:
    // Vec, String, Option, and Result.
}

fn structs() {
    // structs declared using the struct keyword
    struct Vec2 {
        x: f64, // double float
        y: f64,
    }

    // structs initialized using struct literals
    let v1 = Vec2 { x: 1.1, y: 2.2 };
    let v2 = Vec2 { y: 4.4, x: 3.3 }; // order does not matter
    println!("vector1 = {}, {}", v1.x, v1.y);
    println!("vector2 = {}, {}", v2.x, v2.y);

    // the "struct update syntax" is a shortcut for
    // initialing the remaining fields using another
    // struct
    let v3 = Vec2 { x: 42.42, ..v2 };
    println!("vector3 = {}, {}", v3.x, v3.y);

    // "struct update syntax" can be used to update
    // all of the struct fields
    let v4 = Vec2 { ..v3 };
    println!("vector4 = {}, {}", v4.x, v4.y);

    // structs can be destructed like tuples
    let Vec2 { x, y } = v1;
    println!("destructured: {}, {}", x, y);

    // throw away a struct field while destructuring
    let Vec2 { x, .. } = v1;
    println!("partially destructured: {}", x);
}

fn patterns() {
    // define a struct which will get destructured
    struct Number {
        odd: bool,
        val: i32,
    }

    // "if let" is way to (1) pattern match and
    // (2) destructure if the match is true
    fn print_num(n: &Number) {
        // 1. Pattern matching
        //    * Checks if n has its odd field set to true
        // 2. Destructuring
        //    * If true, then destructure the val field
        //      to be used inside the block
        if let Number { odd: true, val } = *n {
            println!("odd number: {}", val);
        } else if let Number { odd: false, val } = *n {
            println!("even number: {}", val);
        }
    }

    let one = Number { odd: true, val: 1 };
    let two = Number { odd: false, val: 2 };
    print_num(&one);
    print_num(&two);

    // match arms are also patterns
    fn print_num_match(n: &Number) {
        match *n {
            Number { odd: true, val } => println!("odd number: {}", val),
            Number { odd: false, val } => println!("even number: {}", val),
        }
    }
    print_num_match(&one);
    print_num_match(&two);

    fn print_num_match_exhaustive(n: &Number) {
        match *n {
            // MATCH MUST BE EXHAUSTIVE!
            // The last match is required or compiler error
            Number { val: 1, .. } => println!("one!"),
            Number { val: 2, .. } => println!("two!"),
            Number { val, .. } => println!("match val = {val}"),
        }
    }
    print_num_match_exhaustive(&one);
    print_num_match_exhaustive(&two);

    fn print_num_match_catch_all(n: &Number) {
        match n.val {
            // MATCH MUST BE EXHAUSTIVE!
            // The last match is required or compiler error
            1 => println!("one!"),
            2 => println!("two!"),
            _ => println!("other val = {}", n.val),
        }
    }
    print_num_match_catch_all(&one);
    print_num_match_catch_all(&two);
    print_num_match_catch_all(&Number { odd: true, val: 11 });
}

fn methods() {
    // can declare methods on your own types
    struct Number {
        odd: bool,
        val: i32,
    }

    impl Number {
        fn is_positive(self) -> bool {
            self.val > 0
        }
    }

    let minus_two = Number {
        odd: false,
        val: -2,
    };
    println!("positive = {}", minus_two.is_positive());
}

fn immutability() {
    // variable bindings are immutable by default;
    // their interior cannot be changed
    struct Number {
        odd: bool,
        val: i32,
    }

    // setting odd would cause the 'cannot assign' error,
    // n is not declared as mutable
    let n = Number { odd: true, val: 17 };
    //n.odd = false;
    println!("n val = {}", n.val);

    // n also cannot be assigned to
    //n = Number { odd: false, val: 12 };

    // mut makes the variable binding mutable
    let mut k = Number { odd: false, val: 2 };
    k.val = 22;
    println!("k val = {}", k.val);
}

fn traits() {
    // our type
    struct Number {
        odd: bool,
        val: i32,
    }

    // traits are something multiple types have in common
    trait Signed {
        fn is_negative(self) -> bool;
    }

    // implementation of the trait
    impl Signed for Number {
        fn is_negative(self) -> bool {
            self.val < 0
        }
    }

    fn print_num(n: &Number) {
        println!("num: odd = {}, val = {}", n.odd, n.val);
    }

    let n = Number {
        odd: false,
        val: -44,
    };
    println!("is negative = {}", n.is_negative());

    // our trait on a foreign type
    impl Signed for i32 {
        fn is_negative(self) -> bool {
            self < 0
        }
    }

    let x: i32 = -44;
    println!("is negative foreign = {}", x.is_negative());

    // foreign trait on our type; the Neg trait
    // is used to overload the '-' operator
    impl std::ops::Neg for Number {
        type Output = Number;

        fn neg(self) -> Number {
            Number {
                val: -self.val,
                odd: self.odd,
            }
        }
    }

    let num = Number {
        odd: true,
        val: 987,
    };
    let neg_num = -num; // works because we impl Neg
    println!("neg_num = {}", neg_num.val);

    // i32 implements the Copy trait
    let a: i32 = 15;
    let b = a; // a is copied
    let c = a; // a is copied again
    println!("a = {}, b = {}, c = {}", a, b, c);

    {
        let n = Number { odd: true, val: 11 };
        // Number does not implement the Copy trait,
        // n is moved into n2
        let n2 = n;
        //let n3 = n; // ERROR: use of moved value: `n`
        print_num(&n2);
    }

    // trait methods can also take self by reference
    // or mutable reference
    impl std::clone::Clone for Number {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }

    {
        // these are equivalent
        let n2 = n.clone();
        let n3 = std::clone::Clone::clone(&n);
        print_num(&n2);
        print_num(&n3);
    }

    {
        // when invoking trait methods, the receiver
        // is borrowed implicitly
        let n = Number { odd: true, val: 51 };
        let mut m = n.clone();
        m.val += 100;
        print_num(&n);
        print_num(&m);
    }

    // marker traits like Copy have no methods
    impl std::marker::Copy for Number {}

    // now Number will no longer be moved since
    // the Copy marker trait is implemented
    {
        let n = Number { odd: true, val: 51 };
        let m = n; // m is a copy of n (not moved or borrowed)
        let o = n; // o is a copy of n (not moved or borrowed)
    }

    // Some traits are so common, they can be
    // implemented automatically by using the
    // derive attribute.
    //
    // This expands to blocks of:
    //    impl Clone for Number2
    //    impl Copy for Number2
    #[derive(Clone, Copy)]
    struct Number2 {
        odd: bool,
        val: i32,
    }
}

fn generics() {}

fn macros() {}

fn panic_macro() {}

fn funcs_that_panic() {}

fn enums() {}

fn lifetimes() {}

fn static_lifetime() {}

fn struct_literal() {}

fn owned_vs_ref_types() {}

fn fallible_functions() {}

fn deferencing() {}

fn function_types() {}

fn loops() {}

fn returning_closures() {}
