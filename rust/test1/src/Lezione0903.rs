use std::io::{stdin, Write};
use std::io::stdout;

pub fn lez(){

    //variables
    println!("Hello, world!");
    let a: i32 = 166;
    let b: i32 = a + 4;
    println!("{}", b);

    //heap
    let mut c = Box::new( (49,80) );
    (*c).0 = 30;
    println!("{:?}", c);

    fn make_box(d: i32) -> Box<(i32,i32)>{
        let b = Box::new((d,2));
        return b;
    }
    let e = make_box(99);
    println!("{:?}", e);

    //arrays
    let f: [i32; 4] = [1,3,7,2];
    let g = [5; 4];
    println!("{:?}", f);
    println!("{:?}", g);

    //print array from n to m
    let g1 = &g[0..2];
    println!("{:?}", g1);

    //strings
    let s0 = String::from("ciao");
    let mut s1 = String::new();
    s1.push_str("meow");
    s1.insert(0, 'B');
    s1.remove(0);
    println!("{}", s0);
    println!("{}", s1);

    //for syntax
    for n in f{
        println!("{}", n);
    }

    //for syntax with strings; iter required
    let names = ["Bob", "Rob", "Tom"];
    for name in names.iter(){
        println!("{}", name);
    }
    for (i,n) in names.iter().enumerate(){
        println!("i:{} - {}",i,n);
    }

    //match to search for multiple values
    //let s: &str = match a{
    //    2  => {"Ciao"}
    //};
    //println!("{}", s);

}

pub fn ecc(){

    print!("Enter a number: ");
    stdout().flush().unwrap();
    let mut input_buffer:String = String::new();
    stdin().read_line(buf:&mut input_buffer).expect(msg: &str);

    let i:i32 = match input_buffer.trim().parse::<i32>() {
        Ok(i:i32) => i,
        Err(e:ParseIntError) => {
            eprintln!("{}: you did not enter a number", e);
            -1
        }
    };
    println!("Integer: {:?}", i);

}
