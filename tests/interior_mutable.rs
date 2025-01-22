#[cfg(test)]

mod interior_mutable_tests {

    use std::cell::Cell;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[tokio::test]
    async fn test_cell_mutable() {
        let data = Cell::new(37);
        data.set(12);
        println!("cell after get {}", data.get());
    }

    // 这里borrow_mut可以用于修改变量，borrow仅用于借用检查
    #[tokio::test]
    async fn test_ref_cell_mutable() {
        let data = RefCell::new(42);
        *data.borrow_mut() += 1;
        println!("ref cell after borrow mut {}", data.borrow());
    }

    #[tokio::test]
    async fn test_ref_cell_share() {
        let data = Rc::new(RefCell::new(42));
        let clone = data.clone();
        *data.borrow_mut() += 1;
        *clone.borrow_mut() += 2;

        assert_eq!(*data.borrow(), 45);
        assert_eq!(*clone.borrow(), 45);
        println!(
            "data borrow {}, clone borrow {}",
            data.borrow(),
            clone.borrow()
        );
    }

    /*
    在 Rust 中，Rc（引用计数，Reference Counting）是一种智能指针，用于在多个地方共享数据的所有权。
    Rc 通过引用计数来管理内存，当最后一个引用被丢弃时，数据会被自动清理。Rc 的主要用途
    共享所有权：
        在 Rust 中，一个值通常只能有一个所有者。Rc 允许你在多个地方共享同一个值的所有权。
        每次调用 Rc::clone 会增加引用计数，每次丢弃 Rc 会减少引用计数。
    避免深拷贝：
        当你需要共享大量数据时，使用 Rc 可以避免深拷贝的开销。
    只读共享：
        Rc 提供的是不可变引用，因此它适用于只读数据的共享。如果需要可变性，可以使用 RefCell 或 Rc<RefCell<T>>。
    */
    #[tokio::test]
    async fn test_interior_mut_reference_counting() {
        {
            let data = Rc::new(32);
            let data_clone1 = Rc::clone(&data);
            let data_clone2 = Rc::clone(&data);
            println!(
                "data {:?}, data_clone1 {:?}, data_clone2 {:?}",
                data, data_clone1, data_clone2
            );
            println!("Reference count: {}", Rc::strong_count(&data));
        }
        println!("-----------------------------------------------------------------");
        {
            let data = Rc::new(vec![1, 2, 3]);
            let data_clone1 = Rc::clone(&data);
            let data_clone2 = Rc::clone(&data);
            println!(
                "data {:?}, data_clone1 {:?}, data_clone2 {:?}",
                data, data_clone1, data_clone2
            );
            println!("Reference count: {}", Rc::strong_count(&data));
        }
        println!("-----------------------------------------------------------------");
        {
            // 每次调用 Rc::clone 会增加引用计数。
            // 每次丢弃 Rc 会减少引用计数。
            // 当引用计数为 0 时，数据会被自动清理。
            let data = Rc::new(36);
            println!(
                "Reference count after creation: {}",
                Rc::strong_count(&data)
            );
            {
                let _ = Rc::clone(&data);
                println!("Reference count after clone 1: {}", Rc::strong_count(&data));
                {
                    let _ = Rc::clone(&data);
                    println!("Reference count after clone 2: {}", Rc::strong_count(&data));
                }
                println!(
                    "Reference count after clone 2 dropped: {}",
                    Rc::strong_count(&data)
                );
            }
            println!(
                "Reference count after clone1 dropped: {}",
                Rc::strong_count(&data)
            );
        }
        println!("-----------------------------------------------------------------");
        {
            let data = Rc::new(RefCell::new(42));
            let clone1 = Rc::clone(&data);

            *data.borrow_mut() += 1;
            println!("data: {}", data.borrow()); // output 43

            *clone1.borrow_mut() += 1;
            println!("data: {}", data.borrow()); // output 44
        }
    }
}
