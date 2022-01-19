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