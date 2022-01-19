mod customer;
mod practice;

fn main() {
    practice::sample::run();

    let input = customer::CustomerRequestDTO {
        id: 1,
        name: String::from("Test"),
        mail: "Test@example.com".to_string(),
        is_active: true,
    };
    println!("{:?}", input);

    let customer = customer::Customer::new(input);
    println!("{:?}", customer);
    println!("{}", customer.id());
    println!("{:p}, {}", customer.name(), customer.name());
    println!("{:p}, {}", customer.mail(), customer.mail());
    println!("{}", customer.is_active());

    let input = customer::CustomerRequestDTO {
        id: 1,
        name: String::from("Test"),
        mail: "Test@example.com".to_string(),
        is_active: true,
    };
    let customer2 = customer::Customer::new(input);
    assert_eq!(customer, customer2);
    if customer == customer2 {
        println!("{:?}", customer);
        println!("{:?}", customer2);
    }
}
