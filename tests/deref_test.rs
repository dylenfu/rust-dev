#[cfg(test)]

mod deref_tests {

    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            println!("hello get into deref test");
            &self.0
        }
    }

    #[tokio::test]
    async fn test_deref() {
        let x = MyBox(12);

        // print "hello get into deref test" twice, because the `assert_eq` and `println` all used `*x`
        assert_eq!(*x, 12);
        println!("deref mybox value {}", *x)
    }
}
