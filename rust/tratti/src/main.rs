struct Pasto{
    name: String,
    cibo: String,
}

trait Mangiare{
    fn cibo(&mut self); 
}

impl Mangiare for Pasto{
    fn cibo(&mut self){
        self.cibo = "Mela".to_string();
    }
}

fn main() {

    let mut pranzo = Pasto{ name:"Pranzo".to_string(), cibo:"".to_string() };
    pranzo.cibo();
    println!("{}", pranzo.cibo);

}
