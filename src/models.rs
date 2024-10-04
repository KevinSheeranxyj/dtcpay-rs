use chrono::NaiveDateTime;

pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub description: Option<String>,
    pub transaction_date: NaiveDateTime,
}


pub struct Product {
    pub id: i32,
    pub name: str,
    address: Address
}

pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String
}
