use std::fmt::{Display, Formatter};
use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dish {
    ThaiChicken,
    Tofu,
    FriedRice,
}

impl Dish {
    fn price(&self) -> u32 {
        match self {
            Dish::ThaiChicken => 20,
            Dish::Tofu => 15,
            Dish::FriedRice => 12,
        }
    }
}

const TAKEAWAY_FEE: u32 = 1;

#[derive(Debug, Clone)]
struct Order {
    dishes: Vec<Dish>,
    is_takeaway: bool,
    count: u32,
    price: u32,
}

impl Order {
    fn new() -> Order {
        Order {
            dishes: Vec::new(),
            is_takeaway: false,
            count: 0,
            price: 0,
        }
    }

    fn add_dish(&mut self, dish: Dish) {
        self.dishes.push(dish);
        self.count += 1;
        self.price += dish.price();
    }

    fn set_takeaway(&mut self) {
        self.is_takeaway = true;
    }

    fn dish_count(&self, dish: Dish) -> u32 {
        let mut x: u32 = 0;
        for y in &self.dishes {
            if y == &dish {
                x += 1;
            }
        }

        x
    }

    fn items_count(&self) -> u32 {
        self.count
    }

    fn is_takeaway(&self) -> bool {
        self.is_takeaway
    }

    fn total(&self) -> u32 {
        let sum = self.price;

        if self.is_takeaway() {
            sum + self.items_count() * TAKEAWAY_FEE
        } else {
            sum
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "chicken: {}, tofu: {}, rice: {}, takeaway: {}",
            self.dish_count(Dish::ThaiChicken),
            self.dish_count(Dish::Tofu),
            self.dish_count(Dish::FriedRice),
            self.is_takeaway()
        )
    }
}

struct Customer {
    name: String,
    favorite_order: Order,
}

impl Customer {
    fn new(name: String, favorite_order: Order) -> Customer {
        Customer {
            name,
            favorite_order,
        }
    }

    fn get_favorite_order(&self) -> Order {
        self.favorite_order.clone()
    }
}

struct KimLoan {
    orders_count: u32,
    customers: Vec<Customer>,
}

impl KimLoan {
    pub fn new() -> KimLoan {
        KimLoan {
            orders_count: 0,
            customers: Vec::new(),
        }
    }

    fn add_customer(&mut self, name: String, favorite_order: Order) {
        self.customers.push(Customer::new(name, favorite_order));
    }

    fn get_saved_customer(&self, name: &str) -> Option<&Customer> {
        self.customers.iter().find(|c| c.name == name)
    }

    fn increase_orders_count(&mut self) {
        self.orders_count += 1;
    }

    fn get_orders_count(&self) -> u32 {
        self.orders_count
    }
}

fn get_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn yes_no(question: &str) -> bool {
    println!("{} (y/n)", question);
    get_line() == "y"
}

fn get_order() -> Order {
    let mut order = Order::new();
    loop {
        println!("Enter dish name or empty line to finish:");
        let line = get_line();
        if line.is_empty() {
            break;
        }
        if line.contains("chicken") {
            order.add_dish(Dish::ThaiChicken);
        } else if line.contains("tofu") {
            order.add_dish(Dish::Tofu);
        } else if line.contains("rice") {
            order.add_dish(Dish::FriedRice);
        } else {
            println!("Unknown dish name: {}", line);
        }
    }

    if yes_no("Takeaway?") {
        order.set_takeaway();
    }

    order
}

fn main() {
    let mut kim_loan = KimLoan::new();

    loop {
        println!("Hi! Welcome to Kim Loan! What's your name?");
        let name = get_line();

        if name.is_empty() {
            break;
        }

        let order = if let Some(customer) = kim_loan.get_saved_customer(&name) {
            println!("Welcome back, {}!", customer.name);
            if yes_no("Same as usual?") {
                customer.get_favorite_order()
            } else {
                get_order()
            }
        } else {
            println!("Welcome, {}!", name);
            let order = get_order();
            if yes_no("Would you like to save this order?") {
                kim_loan.add_customer(name, order.clone());
            }
            order
        };

        if order.items_count() == 0 {
            println!("Your order is empty!");
        } else {
            kim_loan.increase_orders_count();
            println!("This is order no. {}", kim_loan.get_orders_count());
            println!(
                "There you go: {}, it's going to be {} zł",
                order,
                order.total()
            );
        }
    }
    println!("Bye!");
}
