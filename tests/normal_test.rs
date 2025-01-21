#[cfg(test)]

mod normal_tests {

    #[tokio::test]
    async fn test_normal_mut_var() {
        let mut fruit: &str = "apple";
        println!("Testing fruit {}", fruit);
        fruit = "banana";
        println!("{}", fruit);

        let a = 3;
        println!("!a is {}", !a);
    }

    #[tokio::test]
    async fn test_normal_cause() {
        let course = ("programming", "beginner");
        if let ("gaming", c) = course {
            println!("{}", c)
        } else {
            println!("valued unmatched")
        }
    }

    #[tokio::test]
    async fn test_normal_loop_output() {
        let mut i = 1;
        loop {
            println!("{}", i);
            if i == 5 {
                break;
            }
            i += 1;
        }
    }

    /*
    3. Copy 和 Clone 的区别
    特性	Copy	Clone
    复制方式	隐式（自动）	显式（需要调用 .clone()）
    所有权	复制值，不转移所有权	复制值，不转移所有权
    实现复杂度	必须是简单的位复制	可以包含复杂的逻辑（如深拷贝）
    适用类型	简单类型（如整数、浮点数）	复杂类型（如 String、Vec）
    Trait 方法	无方法（标记 trait）	有 clone 方法
    */
    #[tokio::test]
    async fn test_normal_copy_and_clone() {
        #[derive(Copy, Clone, Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p1 = Point { x: 1, y: 2 };
        let p2 = p1; // value copy, but not transfer the ownership
        println!("p1 {:?} p2 {:?}", p1, p2);

        #[derive(Clone, Debug)]
        struct Person {
            name: String,
            age: u8,
        }
        let p1 = Person {
            name: String::from("Alice"),
            age: 12,
        };
        let p2 = p1.clone();
        println!("p1 {:?} p2 {:?}", p1, p2);
    }
}
