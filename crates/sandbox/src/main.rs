use std::any::{type_name, Any, TypeId};

#[derive(Default, Debug)]
pub struct Effect {
    pub name:   String,
    pub damage: i32,
}

#[derive(Default, Debug)]
pub struct Enemy {
    pub hp: u16,
}

#[derive(Default, Debug)]
pub struct Dasd {
    pub hp: u16,
}

#[derive(Default)]
pub struct Data {
    pub fx:    Option<Effect>,
    pub enemy: Option<Enemy>,
}

impl Data {
    pub fn get<T: Any>(self) -> T {
        let raw = match TypeId::of::<T>() {
            id if id == TypeId::of::<Effect>() => Box::new(self.fx.unwrap()) as Box<dyn Any>,
            id if id == TypeId::of::<Enemy>() => Box::new(self.enemy.unwrap()) as Box<dyn Any>,
            _ => panic!("cannot get type {:?}", type_name::<T>()),
        };

        *(raw.downcast::<T>().ok().unwrap())
    }
}

fn main() {
    let effect_data = Data {
        fx: Some(Effect {
            damage: 13,
            name:   String::from("fire ball"),
        }),
        ..Default::default()
    };

    // i know that it is effect
    let fx = effect_data.get::<Dasd>();
    println!("{:?}", fx)
}
