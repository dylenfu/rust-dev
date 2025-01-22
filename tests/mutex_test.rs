#[cfg(test)]

mod mutex_tests {

    use std::sync::{Arc, Mutex, RwLock};
    use std::thread;

    /*
    Mutex<T>
        作用：Mutex（互斥锁）提供独占访问，即同一时间只允许一个线程访问数据。
        对 T 的要求：
        T 不需要实现 Sync，因为 Mutex 内部已经提供了同步机制。
        T 可以是任何类型，但通常需要是 Send，因为 Mutex 本身是 Send 和 Sync 的，允许跨线程传递和共享。

        访问模式：
        通过 lock 方法获取锁，返回一个 MutexGuard，提供对 T 的独占访问。
        如果另一个线程已经持有锁，当前线程会阻塞，直到锁被释放。
    */
    #[tokio::test]
    async fn test_mutex_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let clone = counter.clone();
            let handler = thread::spawn(move || {
                let mut mut_counter = clone.lock().unwrap();
                *mut_counter += 1;
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
        println!("after counter = {}", *counter.lock().unwrap());
    }

    /*
    RwLock<T>
        作用：RwLock（读写锁）提供更灵活的并发访问模式，允许多个读操作或一个写操作。
        对 T 的要求：
        T 不需要实现 Sync，因为 RwLock 内部已经提供了同步机制。
        T 可以是任何类型，但通常需要是 Send，因为 RwLock 本身是 Send 和 Sync 的，允许跨线程传递和共享。

        访问模式：
        通过 read 方法获取读锁，返回一个 RwLockReadGuard，允许多个线程同时读取数据。
        通过 write 方法获取写锁，返回一个 RwLockWriteGuard，允许一个线程独占写访问。
        如果另一个线程已经持有写锁，当前线程会阻塞，直到写锁被释放。
        如果另一个线程持有读锁，写锁会阻塞，直到所有读锁被释放。
    */
    #[tokio::test]
    async fn test_rwlock_data() {
        let data = Arc::new(RwLock::new(0));
        let mut handlers = vec![];

        // reading threads
        for _ in 0..10 {
            let clone = data.clone();
            let handler = thread::spawn(move || {
                let guard = clone.read().unwrap();
                println!("read rwlock guard {:?}", *guard);
            });
            handlers.push(handler);
        }

        // writing threads
        for _ in 0..10 {
            let clone = data.clone();
            let handler = thread::spawn(move || {
                let mut guard = clone.write().unwrap();
                *guard += 1;
                println!("write rwlock guard {:?}", *guard);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("after data = {}", *data.read().unwrap());
    }
}
