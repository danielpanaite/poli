fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(19);
    println!("{:?}", v);

    let i = 42;
    let j = i+2;
    println!("{}", i);

    let mut z = 10;
    let c = &mut z;
    //z = 20;
    println!("{}", c); 
}
