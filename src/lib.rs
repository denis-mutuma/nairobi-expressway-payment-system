use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;

// the max fees to be paid by an unregistered vehicle that uses the expressway
const MAX_FEES: i32 = 360;

// a vehicle should be categorised as one approved to use the expressway
#[derive(BorshDeserialize, BorshSerialize)]
enum VehicleType {
    SALOON,
    BUS,
    TRAILER,
    OTHER,
}

// vehicle attributes
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Vehicle {
    types: VehicleType,
    reg_no: String,
    from_to: String,
}

// the card struct defines the card attributes
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Card {
    balance: i32,
    user: String,
}

// Our main App has these data structures
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct App {
    // CONTRACT STATE
    route_fares: HashMap<String, i32>, // a map of toll fees for each route (key: entry and exit point, value: toll fees)
    card: Vec<Card>,                   // a vector of all registered cards
    car: Vec<Vehicle>,                 // a vector of all registered vehicles
}

// the implementation of the main application and related methods
#[near_bindgen]
impl App {
    // CONTRACT METHODS

    // new returns an instance of our applications with predefined contents for the App data structures
    #[init]
    #[private]
    fn new() -> Self {
        // map each route to the toll fees for that route
        let mut route: HashMap<String, i32> = HashMap::new();

        // populate the route with the data. Source: https://nairobiexpressway.ke/
        route.insert("westlands-mlolongo".to_string(), 360);
        route.insert("mlolongo-westlands".to_string(), 360);

        route.insert("westlands-syokimau".to_string(), 360);
        route.insert("syokimau-westlands".to_string(), 360);

        route.insert("westlands-jkia".to_string(), 300);
        route.insert("jkia-westlands".to_string(), 300);

        route.insert("westlands-easternBypass".to_string(), 300);
        route.insert("easternBypass-westlands".to_string(), 300);

        route.insert("westlands-southernBypass".to_string(), 240);
        route.insert("southernBypass-westlands".to_string(), 240);

        route.insert("westlands-capitalCenter".to_string(), 180);
        route.insert("capitalCenter-westlands".to_string(), 180);

        route.insert("westlands-haileSelassie".to_string(), 180);
        route.insert("haileSelassie-westlands".to_string(), 180);

        route.insert("westlands-museumHill".to_string(), 120);
        route.insert("museumHill-westlands".to_string(), 120);

        return App {
            route_fares: route,
            card: vec![],
            car: vec![],
        };
    }

    // new_car creates an instance of the vehicle type and appends it to the car vector
    fn new_car(&mut self, from_to: String, types: String, reg_no: String) {
        let _sl = "saloon".to_string();
        let _bs = "bus".to_string();
        let _tl = "trailer".to_string();

        for elem in self.car.iter() {
            if elem.reg_no == reg_no {
                env::log_str("Could not create car. Car already exists!");
                return;
            }
        }

        let cr = Vehicle {
            types: match types {
                _sl => VehicleType::SALOON,
                _bs => VehicleType::BUS,
                _tl => VehicleType::TRAILER,
                _ => VehicleType::OTHER,
            },
            reg_no: reg_no,
            from_to: from_to,
        };
        self.car.push(cr);
    }

    // amount_to_pay returns the toll fees for each trip
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
                    None => MAX_FEES,
                }
            }
            None => {
                env::log_str("Pay max fare because car not found");
                MAX_FEES
            }
        };
    }

    // pay deducts the toll fees from the card.
    // if card balance is not sufficient, the user will be prompted to pay with cash
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

    // register_card creates a new card and appends it to the cards vector
    fn register_card(&mut self, account: String) {
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

    // top_up updates the balance of an existing card to balance + amount deposited
    fn top_up(&mut self, amount: i32, account: String) {
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

// unit tests begin here
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

    // new should return a hashmap of length 16 and
    // two empty vectors: card vector and cars vector, each of length 0
    #[test]
    fn new() {
        let app = App::new();
        assert_eq!(app.route_fares.len(), 16);
        assert_eq!(app.card.len(), 0);
        assert_eq!(app.car.len(), 0);
    }

    // new_car should create a new car object and append it to the cars vector
    // The length of cars vector should increase by one for each new car object created
    // The car reg_no must be unique. If car reg_no already exists, no new car should be created
    #[test]
    fn new_car() {
        let mut app = App::new();

        // adding a new car should work
        app.new_car(
            "westlands-mlolongo".to_string(),
            "saloon".to_string(),
            "KDJ 001A".to_string(),
        );
        assert_eq!(app.car.len(), 1);
        assert_eq!(app.car[0].reg_no, "KDJ 001A");

        // registering a car more than once should fail and the length of car vector
        // should not change
        app.new_car(
            "westlands-syokimau".to_string(),
            "bus".to_string(),
            "KDJ 001A".to_string(),
        );
        assert_eq!(app.car.len(), 1);

        // adding a different car should work
        app.new_car(
            "westlands-easternBypass".to_string(),
            "trailer".to_string(),
            "KDK 001A".to_string(),
        );
        assert_eq!(app.car.len(), 2);
        assert_eq!(app.car[1].reg_no, "KDK 001A");
    }

    // amount_to_pay should return the amaount to be paid for each trip
    #[test]
    fn amount_to_pay() {
        let mut app = App::new();
        app.new_car(
            "westlands-mlolongo".to_string(),
            "saloon".to_string(),
            "KDJ 001A".to_string(),
        );

        app.new_car(
            "westlands-southernBypass".to_string(),
            "saloon".to_string(),
            "KDK 001A".to_string(),
        );

        let amount_car1 = app.amount_to_pay("KDJ 001A".to_string());
        let amount_car2 = app.amount_to_pay("KDK 001A".to_string());
        assert_eq!(amount_car1, 360); // amount should match each trip
        assert_eq!(amount_car2, 240); // amount should match each trip

        // an unregistered car should pay 360 i.e. MAX_FEES by default
        let amount_uregistered = app.amount_to_pay("GKB 001Y".to_string());
        assert_eq!(amount_uregistered, 360);
    }

    #[test]
    // register_card should register a new card if the card is not registered.
    // a card should only register once
    fn register_card() {
        let mut app = App::new();

        app.register_card("John Doe".to_string());
        app.register_card("Ken Thompson".to_string());

        assert_eq!(app.card[0].user, "John Doe");
        assert_eq!(app.card[0].balance, 0);

        assert_eq!(app.card[1].user, "Ken Thompson");
        assert_eq!(app.card[1].balance, 0);
    }

    // top up should update the account balance to balance + top up amount
    #[test]
    fn top_up() {
        let mut app = App::new();

        app.register_card("John Doe".to_string());
        assert_eq!(app.card[0].balance, 0);

        app.top_up(100, "John Doe".to_string());
        assert_eq!(app.card[0].balance, 100);

        app.top_up(100, "John Doe".to_string());
        assert_eq!(app.card[0].balance, 200);
    }

    // pay should update the account balance by deducting the amount to be paid for each trip
    #[test]
    fn pay() {
        let mut app = App::new();
        app.new_car(
            "westlands-mlolongo".to_string(),
            "saloon".to_string(),
            "KDJ 001A".to_string(),
        );

        app.register_card("John Doe".to_string());
        app.top_up(1000, "John Doe".to_string());
        assert_eq!(app.card[0].balance, 1000);

        let mut amount = app.amount_to_pay("KDJ 001A".to_string());
        assert_eq!(amount, 360);

        app.pay("KDJ 001A".to_string(), "John Doe".to_string());
        assert_eq!(app.card[0].balance, 640);

        app.pay("KDJ 001A".to_string(), "John Doe".to_string());
        assert_eq!(app.card[0].balance, 280);
    }
}
