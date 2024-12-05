use serde::Deserialize;

#[derive(Deserialize)]
pub struct Account {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub postcode: String,
    pub country: String,
}
