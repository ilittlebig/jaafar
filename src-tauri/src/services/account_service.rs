use serde::Deserialize;
use fake::faker::address::en::*;
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::Fake;
use rand::Rng;

#[derive(Deserialize, Clone, Debug)]
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

impl Account {
    pub fn populate_random_fields(&mut self) {
        if self.firstname.to_lowercase() == "random" {
            self.firstname = FirstName().fake();
        }

        if self.lastname.to_lowercase() == "random" {
            self.lastname = LastName().fake();
        }

        if self.phone.to_lowercase() == "random" {
            self.phone = PhoneNumber().fake();
        }

        if self.address1.to_lowercase() == "random" {
            self.address1 = StreetName().fake();
        }

        if self.address2.to_lowercase() == "random" {
            self.address2 = if rand::random() {
                format!("Apt {}", rand::random::<u16>() % 1000)
            } else {
                String::new()
            };
        }

        if self.city.to_lowercase() == "random" {
            self.city = CityName().fake();
        }

        if self.postcode.to_lowercase() == "random" {
            self.postcode = PostCode().fake();
        }
    }
}
