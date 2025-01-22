#[cfg(test)]

mod sized_tests {

    /*
        在 Rust 中，Sized 是一个标记 trait（marker trait），用于表示一个类型的大小在编译时是已知的。这是 Rust 类型系统中的一个重要概念，
        因为它影响了如何存储、传递和使用类型。

        1. Sized 的含义
        编译时已知大小：
        如果一个类型实现了 Sized，那么它的内存大小在编译时是已知的。
        例如，i32、u64、bool 等基本类型都是 Sized 的，因为它们的大小在编译时是固定的。
        默认约束：
        在 Rust 中，泛型类型参数默认是 Sized 的。例如：

        rust
        复制
        fn foo<T>(t: T) {}
        等价于：

        rust
        复制
        fn foo<T: Sized>(t: T) {}
        动态大小类型（DST）：

        有些类型的大小在编译时是未知的，称为动态大小类型（Dynamically Sized Types, DST）。例如：
        切片（[T]）：切片的大小取决于运行时的长度。
        trait 对象（dyn Trait）：trait 对象的大小取决于具体的实现类型。
        这些类型没有实现 Sized。

        2. Sized 的作用
        存储和传递：
        Sized 类型可以存储在栈上，也可以作为函数参数和返回值传递。
        非 Sized 类型（如 [T] 或 dyn Trait）必须通过指针（如 &[T] 或 Box<dyn Trait>）来间接存储和传递。
        泛型约束：
        如果你需要处理可能非 Sized 的类型，可以显式地放宽 Sized 约束：
        rust
        复制
        fn foo<T: ?Sized>(t: &T) {}
        这里的 ?Sized 表示 T 可以是 Sized 或非 Sized 的。
    */

    #[tokio::test]
    async fn test_print_sized() {
        let x: i32 = 51;
        print_sized(x);
    }

    #[tokio::test]
    async fn test_print_unsized() {
        let x: Vec<i32> = vec![1, 2, 3, 4];
        // printed pointer length but not i32 length
        print_unsized(&x);
    }

    #[tokio::test]
    async fn test_print_due_sized() {
        let x: i32 = 51;
        print_sized(x);

        let y: Vec<i32> = vec![1, 2, 3, 4];
        print_unsized(&y);
    }

    fn print_sized<T: Sized>(_t: T) {
        println!("Size of T: {}", std::mem::size_of::<T>());
    }

    fn print_unsized<T: ?Sized>(t: &T) {
        println!("Unsized T: {}", std::mem::size_of_val(&t));
    }
}
