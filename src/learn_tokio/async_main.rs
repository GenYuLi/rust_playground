mod socket_ext;

use std::sync::Arc;
use std::time::Duration;

use socket_ext::test_read_u8_timeout_error_timeout;

use tokio::io::AsyncReadExt;
use tokio::task;
use tokio::time::{self, sleep};

use log::Level;

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

async fn sleeper() {
    log::info!("Sleeping!");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading some beeg data");
    let mut f = tokio::fs::File::open("beeg.txt").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read {} bytes", contents.len());

    // This will take a while to compute
    // fib(40)

    // This will not block the tokio runtime because it's running on a blocking thread pool
    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(40)");
        fib(40);
        log::info!("Done computing fib(40)");
    })
    .await
    .unwrap();
}

async fn do_something_fun() {
    log::info!("Doing something fun!");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Done doing something fun!");
}

async fn do_something_very_Fun() {
    log::info!("Doing something very fun!");
    time::sleep(time::Duration::from_secs(2)).await;
    log::info!("Done doing something very fun!");
}

async fn run() {
    /*tokio::join! {
        sleeper(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
    };*/
    // tokio::spawn(async {
    // sleeper().await;
    // });
    // tokio::spawn(async {
    // do_something_fun().await;
    // });
    // do_something_very_Fun().await;
    // can i await here without any function call?
    // no, because the async block is not a future

    //sleeper_future.await;
    //reader_future.await;
}

// without [tokio::main]
fn main_without_tokio() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    let future = run();

    rt.spawn(sleeper());
    rt.spawn(do_something_fun());
    rt.block_on(do_something_very_Fun());
}

struct MyStruct {
    value: i32,
}

impl MyStruct {
    async fn do_something(self: &Self) {
        // 這裡使用的是 Arc 的 clone，並且整個操作不借用非 'static 的資料
        println!("Value is {}", self.value);
        sleep(Duration::from_secs(1)).await;
    }

    fn spawn_task(self: Arc<Self>) {
        // 將 self 移入 async move block 中，這樣整個 future 是 'static
        task::spawn(async move {
            self.do_something().await;
        });
    }
}

async fn excample_b() {
    let s = String::from("hi");
    // let f = move || async {
    //    println!("A: {}", s);
    // };
}

async fn example_a() -> impl std::future::Future<Output = String> {
    let s = String::from("hello");

    let f = || async move {
        println!("B: {}", &s);
        s
    };

    f()
}

// with [tokio::main]
#[tokio::main]
async fn main() {
    let my_struct = MyStruct { value: 42 };
    tokio::spawn({
        async move {
            my_struct.do_something().await;
        }
    });

    let err = test_read_u8_timeout_error_timeout().await;
    dbg!(&err);
    // take err as a result, and print it
    if let Err(e) = &err {
        dbg!(e);
    } else {
        eprintln!("No error");
        let data = err.unwrap();
        eprintln!("Data: {:?}", data);
    }
    dbg!("done");
    simple_logger::init_with_level(Level::Info).unwrap();
    let example_a = example_a().await;
    let var_name = example_a.await;
    println!("{var_name}");
    // tokio::select!{
    // _ = time::sleep(time::Duration::from_secs(1)) => {
    // log::info!("Slept for 1 second");
    // }
    // _ = time::sleep(time::Duration::from_secs(2)) => {
    // log::info!("Slept for 2 seconds");
    // }
    // }
    let t = async move || async {};
    let k = move || async move {};

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Time elapsed: {:?}", end - start);
}
