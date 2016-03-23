// container object must not live longer than the object that it holds a reference to.
struct Car {
    model: String,
}

pub struct Person<'a> {
    car: Option<&'a Car>,
}

impl<'a> Person<'a> {
    fn new() -> Person<'a> {
        Person { car: None }
    }

    fn buy_car(&mut self, c: &'a Car) {
        self.car = Some(c);
    }

    fn sell_car(&mut self) {
        self.car = None;
    }

    fn trade_with(&mut self, other: &mut Person<'a>) {
        let tmp = other.car;
        other.car = self.car;
        self.car = tmp;
    }
}

fn main() {
    let honda = Car { model: "Honda".to_string() };
    let toyota = Car { model: "Toyota".to_string() };
    let mut bob = Person::new();
    let mut alice = Person::new();
    bob.buy_car(&honda);
    bob.buy_car(&toyota);
    // bob.trade_with(&mut alice);
    let p1 = &honda;
}
