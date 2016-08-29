struct Pizza(Vec<i32>);
struct PizzaSlice<'a>{pizza: &'a Pizza, index: i32}
struct PizzaConsumer<'a, 'b:'a> {
    slice: PizzaSlice<'a>,
    pizza: &'b Pizza,
}

fn get_another_slice(c: &mut PizzaConsumer, index: i32) {
    c.slice = PizzaSlice {pizza: c.pizza, index: index};
}

let p:Pizza = Pizza(vec![1,2,3,4]);
{
    let s = PizzaSlice{pizza: &p, index: 1};
    let mut c = PizzaConsumer{slice: s, pizza: &p};
    get_another_slice(&mut c, 2);
}
