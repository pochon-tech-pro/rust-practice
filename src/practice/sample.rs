use std::cell::RefCell;
use std::collections::HashMap;
use crate::practice::customer::{Customer, CustomerCollection};
// super: ここでは、fileが同じディレクトリにいるのでわかりづらいが、lib(main)にぶら下がるpractice。
// superは予約語で..に相当する。全て読み込みたいのであればワイルドカードを指定する。
use super::customer;

pub fn run() {
    let input = customer::CustomerRequestDTO {
        id: 1,
        name: String::from("Test"),
        mail: "Test@example.com".to_string(),
        is_active: true,
    };
    println!("{:?}", input);

    // Customerインスタンスの生成
    let customer = customer::Customer::new(input);
    println!("{:?}", customer);
    println!("{}", customer.id());
    println!("{:p}, {}", customer.name(), customer.name());
    println!("{:p}, {}", customer.mail(), customer.mail());
    println!("{}", customer.is_active());

    // Customerインスタンスの比較
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

    // HashMapを使ってみる
    let input = customer::CustomerRequestDTO {
        id: 2,
        name: String::from("Test2"),
        mail: "Test2@example.com".to_string(),
        is_active: true,
    };
    let customer3 = customer::Customer::new(input);
    let mut customers: HashMap<u32, customer::Customer> = HashMap::new();

    customers.insert(customer.id().clone(), customer);
    customers.insert(customer3.id().clone(), customer3);

    show_customers(&customers);
    show_customers(&customers);

    // Vector + Collection Objectを体験してみる
    let mut customer_collection = CustomerCollection::new();
    let input = customer::CustomerRequestDTO {
        id: 1001,
        name: String::from("John Smith"),
        mail: "John@example.com".to_string(),
        is_active: true,
    };
    customer_collection.add(customer::Customer::new(input));
    customer_collection.show();
    customer_collection.remove();
    customer_collection.show();
}

fn show_customers (customers: &HashMap<u32, customer::Customer>) {
    println!("{:?}", customers);
}