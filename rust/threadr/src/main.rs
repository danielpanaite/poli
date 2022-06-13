use std::thread;
use std::sync::{Arc, RwLock};

fn main(){

    //Basic thread

    /* let thread_join_handle = thread::spawn(move ||  {
        println!("Thread executes Meow!");
        return 0; //return success
    });
    
    match thread_join_handle.join(){
        Ok(res) => {println!("Result:{:?}", res)},
        Err(err) => {println!("Error:{:?}", err)},
    } */

    /* let builder = thread::Builder::new().name("meow1".to_string()).stack_size(100_000);
    let handler = builder.spawn(|| {println!("Thread meow1 started")}).unwrap();

    handler.join().unwrap(); */

    //let sharedmap = Arc::new(Mutex::new(HashMap::new()));
    let str0 = "Ciao!";
    let sharedin = Arc::new(RwLock::new(str0));
    let mut threads = vec![];

    for i in 0..8 {
        //let mut data = sharedmap.clone();
        let inlock = Arc::clone(&sharedin);

        threads.push(thread::spawn(move || {
            //let mut v = data.lock().unwrap();
            let inp = inlock.read();
            println!("Thread {} - {:?}", i, inp);
        }).join().unwrap());
    }
    
}
