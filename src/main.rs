mod customer;

fn main() {
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
}
