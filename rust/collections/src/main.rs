#[derive(Debug)]

enum IpAddr{
    V4,
    V6,
}

fn main() {
    println!("Collection tests v1");

    let area = vec![
        IpAddr::V4,
        IpAddr::V4,
        IpAddr::V6,
    ];

    println!("{:?}", area);

}
