use std::env;

fn capitalize(s: &str) -> String {

    let mut c: Vec<char> = s.chars().collect();
    if s.is_empty() {
        return "".to_string();
    }
    for n in 0..c.len(){
        if n == 0 {
            let upper: Vec<_> = c[0].to_uppercase().collect();
            c[0] = upper[0];
        }
        if c[n] == ' ' {
            let upper: Vec<_> = c[n+1].to_uppercase().collect();
            c[n+1] = upper[0];
        }
    }
    let ret: String = c.iter().collect();
    return ret;

}

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{}",capitalize(&args[1]));
}
