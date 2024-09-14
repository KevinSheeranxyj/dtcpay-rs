

pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub description: Option<String>,
    pub transaction_date: NaiveDateTime,
}