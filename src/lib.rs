use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;

const MAX_FARE: i32 = 360;

#[derive(BorshDeserialize, BorshSerialize)]
enum VehicleType {
    SALOON,
    BUSES,
    TRAILERS,
    OTHER,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Vehicle {
    types: VehicleType,
    reg_no: String,
    from_to: String,
    // from: String,
    // to: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Card {
    balance: i32,
    user: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct App {
    // SETUP CONTRACT STATE
    route_fares: HashMap<String, i32>,
    card: Vec<Card>,
    car: Vec<Vehicle>,
}

#[near_bindgen]
impl App {
    // ADD CONTRACT METHODS HERE
    #[init]
    #[private]
    fn new() -> Self {
        let mut route: HashMap<String, i32> = HashMap::new();

        route.insert("westlands-mlolongo".to_string(), 360);
        route.insert("westlands-syokimau".to_string(), 360);
        route.insert("westlands-jkia".to_string(), 300);
        route.insert("westlands-easternBypass".to_string(), 300);
        route.insert("westlands-southernBypass".to_string(), 240);
        route.insert("westlands-capitalCenter".to_string(), 180);
        route.insert("westlands-haileSelassie".to_string(), 180);
        route.insert("westlands-museumHill".to_string(), 120);

        return App {
            route_fares: route,
            card: vec![],
            car: vec![],
        };
    }

    fn new_car(&mut self, from_to: String, types: String, reg_no: String) {
        // let from_to = self.from + "-" + self.to;
        // let cost = route.get(from_to);
        // cost
        let _sl = "saloon".to_string();
        let _bs = "buses".to_string();
        let _tl = "trailers".to_string();

        let cr = Vehicle {
            types: match types {
                _sl => VehicleType::SALOON,
                _bs => VehicleType::BUSES,
                _tl => VehicleType::TRAILERS,
                _ => VehicleType::OTHER,
            },
            reg_no: reg_no,
            from_to: from_to,
        };
        self.car.push(cr);
    }

    fn amount_to_pay(&self, reg_no: String) -> i32 {
        let mut car: Option<&Vehicle> = None;

        for elem in self.car.iter() {
            if elem.reg_no == reg_no {
                car = Some(elem);
                break;
            }
        }

        return match car {
            Some(cr) => {
                let amount = self.route_fares.get(&cr.from_to);

                match amount {
                    Some(sm) => *sm,
                    None => MAX_FARE,
                }
            }
            None => {
                env::log_str("Pay max fare because car not found");
                MAX_FARE
            }
        };
    }

    fn pay(&mut self, reg_no: String, account: String) -> String {
        let amount = self.amount_to_pay(reg_no);

        let mut info = String::from("unknown");
        for card_item in self.card.iter_mut() {
            if card_item.user == account {
                if card_item.balance == amount || card_item.balance > amount {
                    card_item.balance -= amount;

                    "okay".to_string();
                } else {
                    let ft = format!("Please pay in cash, card has {}", card_item.balance);
                    env::log(ft.as_bytes());

                    info = "error".to_string();
                }
            }
        }

        info
    }

    fn register_card(&mut self, account: String) {
        //    self.card.iter().con(predicate)
        let mut card_exist = false;
        for card_item in self.card.iter_mut() {
            if card_item.user == account {
                card_exist = true;
            }
        }

        if card_exist {
            env::log_str("error card exist");
        } else {
            let cd = Card {
                user: account,
                balance: 0,
            };
            self.card.push(cd);
        }
    }

    fn top_up(&mut self, amount: i32, account: String) {
        // self.balance += amount;
        // println!(
        //     "Deposited {} int account. New balance is KES {}",
        //     amount, self.balance
        // );
        let mut card_exist = false;
        for card_item in self.card.iter_mut() {
            if card_item.user == account {
                card_exist = true;

                card_item.balance += amount;
            }
        }

        if !card_exist {
            env::log_str("error  card does not exist");
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE

    // new should return a hashmap of size 8 and
    // two empty vectors: card and cars
    // #[test]
    // fn new() {
    //     let app = App::new();
    //     assert_eq!(app.route_fares.capacity(), 8);
    //     assert_eq!(app.card.capacity(), 0);
    //     assert_eq!(app.car.capacity(), 0);
    // }

    // new car should append a new car to the cars vector
    #[test]
    // fn new_car{(from_to: "westlands-mlolongo", types: "saloon", reg_no: "KDA 001A") {
    fn new_car() {
        let mut app = App::new();
        app.new_car(
            "westlands-mlolongo".to_string(),
            "saloon".to_string(),
            "KDA 001A".to_string(),
        );
        assert_eq!(app.car.len(), 1);
    }

    //
    #[test]
    fn amount_to_pay() {
        let mut app = App::new();
        app.new_car(
            "westlands-mlolongo".to_string(),
            "saloon".to_string(),
            "KDA 001A".to_string(),
        );

        let amount = app.amount_to_pay("KDA 001A".to_string());
        assert_eq!(amount, 360);
    }

    // #[test]
    // fn pay(reg_no: String, account: String) -> String {}
}
