#[derive(Debug)]

// Player
struct Player{
    name: String,
    class: String,
    health: i32,
    mana: i32,
    level: u32,
    xp: u32,
}

impl Player{
    fn stats(&self){
        println!("Player {} is level {} with {} xp", self.name, self.level, self.xp);
        println!{"Stats: {} - HP {} MP {}", self.class, self.health, self.mana};
    }

    fn win(&self, enemylvl: u32) -> u32{
        let xp: u32 = enemylvl * 60;
        return xp
    }
}

// Meow 
struct Meow{
    value: i32,
    number: i32,
}

impl Drop for Meow{
    fn drop(&mut self){
        println!("Variable deleted");
    }
}

enum HttpResponse{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalError = 500
}

// Main function
fn main() {
    println!(" Struct - 0.1");
    println!("--------------");

    //Classes
    let mage = Player{ name: "Name".to_string(), class: "Mage".to_string(), health: 90, mana: 120, level: 1, xp: 0 };
    let warrior = Player{ name: "Name".to_string(), class: "Warrior".to_string(), health: 110, mana: 80, level: 1, xp: 0 };

    let gatto = Meow{ value: 100, number: 1 };

    //Players
    let mut p1 = Player{ name: "Wamuu".to_string(), .. mage };
    let mut e1 = Player{ name: "Dio".to_string(), .. warrior };
    e1.level = 5;

    //Methods
    p1.xp += p1.win(e1.level);
    p1.stats();

    println!("Meow {}", gatto.value);
    drop(gatto);
    //println!("{}", gatto.number); //generates error

}
