#[derive(Debug)]
pub struct CustomerRequestDTO {
    pub id: u32,
    pub name: String,
    pub mail: String,
    pub is_active: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Customer {
    id: u32,
    name: String,
    mail: String,
    is_active: bool,
}

impl Customer {
    pub fn new(input: CustomerRequestDTO) -> Customer {
        Customer { id: input.id, name: input.name, mail: input.mail, is_active: input.is_active }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn mail(&self) -> &String {
        &self.mail
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

pub struct CustomerCollection {
    list: Vec<Customer>,
}

impl CustomerCollection {
    pub fn new() -> CustomerCollection {
        CustomerCollection { list: vec![] }
    }
    pub fn show(&self) {
        println!("{:?}", self.list);
    }
    pub fn add(&mut self, value: Customer) {
        self.list.push(value);
    }
    pub fn remove(&mut self) -> Option<Customer> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                Some(value)
            },
            None => None,
        }
    }
}
