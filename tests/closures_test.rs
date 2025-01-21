#[cfg(test)]

mod closures_tests {

    #[tokio::test]
    async fn test_closure_fn() {
        let mut x = 5;
        let mut increment_x = || {
            x += 1;
            println!("x is now {}", x);
        };

        increment_x();  // output the x should be 6
        increment_x();  // output the x should be 7
    }
}