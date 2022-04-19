use Index;

struct Pasto{
    name: String,
    cibo: String,
}

trait Mangiare{
    fn gatto(&self){
        println!("Meow!");
    }
}

impl Index for Pasto{}

fn main() {

    let mut pranzo = Pasto{ name:"Pranzo".to_string(), cibo:"".to_string() };

}
