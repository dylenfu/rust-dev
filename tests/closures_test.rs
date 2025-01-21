#[cfg(test)]

mod closures_tests {

    // the example of closure `Fn` show us that the var `x`
    // in closure are not mutable, so can be called multi times
    #[tokio::test]
    async fn test_closure_fn() {
        let x = 5;
        let print_x = || println!("x is {}", x); // can not add x += 1, throw compile error

        print_x();
        print_x();
    }

    // the mutable `x` can be changed multi times because of `FnMut`
    #[tokio::test]
    async fn test_closure_fn_mut() {
        let mut x = 5;
        let mut increment_x = || {
            x += 1;
            println!("x is now {}", x);
        };

        increment_x(); // output the x should be 6
        increment_x(); // output the x should be 7
    }

    // closure of FnOnce can be called multi times, because that the variable of `x` not changed
    #[tokio::test]
    async fn test_closure_fn_once() {
        let x = vec![1, 2, 3];
        let consume_x = move || {
            println!("x is {:?}", x);
        };
        consume_x(); // output: x is [1, 2, 3]
        consume_x(); // output: x is [1, 2, 3]
    }

    // example to show us that the trait of `FnOnce` can only be called once
    // because the ownership of var `x` have changed
    #[tokio::test]
    async fn test_closure_fn_once_mut() {
        let x = vec![1, 2, 3];
        let consume_x = move || {
            println!("x is {:?}", x);
            drop(x)
        };

        consume_x();
        // consume_x(); // throw compile error because the ownership of `x` has changed
    }
}
