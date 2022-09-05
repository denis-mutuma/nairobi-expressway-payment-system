use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Promise;
use near_sdk::{env, near_bindgen};

#[warn(dead_code)]
fn one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
}

// the max fees to be paid by an unregistered vehicle that uses the expressway
// const MAX_FEES: i32 = 360;

// a vehicle should be categorised as one approved to use the expressway
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize,)]
#[serde(crate = "near_sdk::serde")]
enum VehicleType {
    SALOON,
    BUS,
    TRAILER,
    OTHER,
}

// vehicle attributes
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize ,Debug, Serialize, Deserialize,)]
#[serde(crate = "near_sdk::serde")]
pub struct Vehicle {
    types: VehicleType,
    reg_no: String,
    from_to: String,
}

// Our main App has these data structures
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct App {
    // CONTRACT STATE
    route_fares: UnorderedMap<String, u128>, // a map of toll fees for each route (key: entry and exit point, value: toll fees)

    car: Vector<Vehicle>, // a vector of all registered vehicles
}

/// new returns an instance of our applications with predefined contents for the App data structures
impl Default for App {
    fn default() -> Self {
        // map each route to the toll fees for that route
        let mut route: UnorderedMap<String, u128> = UnorderedMap::new(b"s".to_vec());

        // populate the route with the data. Source: https://nairobiexpressway.ke/
        route.insert(&"westlands-mlolongo".to_string(), &one_near());
        route.insert(&"mlolongo-westlands".to_string(), &(one_near() * 1));

        route.insert(&"westlands-syokimau".to_string(), &(one_near() * 1));
        route.insert(&"syokimau-westlands".to_string(), &(one_near() * 1));

        route.insert(&"westlands-jkia".to_string(), &(one_near() * 1));
        route.insert(&"jkia-westlands".to_string(), &(one_near() * 1));

        route.insert(&"westlands-easternBypass".to_string(), &(one_near() * 1));
        route.insert(&"easternBypass-westlands".to_string(), &(one_near() * 1));

        route.insert(&"westlands-southernBypass".to_string(), &(one_near() * 1));
        route.insert(&"southernBypass-westlands".to_string(), &(one_near() * 1));

        route.insert(&"westlands-capitalCenter".to_string(), &(one_near() * 1));
        route.insert(&"capitalCenter-westlands".to_string(), &(one_near() * 1));

        route.insert(&"westlands-haileSelassie".to_string(), &(one_near() * 1));
        route.insert(&"haileSelassie-westlands".to_string(), &(one_near() * 1));

        route.insert(&"westlands-museumHill".to_string(), &(one_near() * 1));
        route.insert(&"museumHill-westlands".to_string(), &(one_near() * 1));

        return App {
            route_fares: route,
            car: Vector::new(b"r".to_vec()),
        };
       
    }
}

// the implementation of the main application and related methods
#[near_bindgen]
impl App {
    
    
    #[private]
    pub  fn max_fare(&self) -> u128 {
        one_near() * 10
    }

    pub  fn get_cars(&self) -> Vec<Vehicle> {
        self.car.to_vec()
    }

    fn get_car_type(&self, car_type:String)->VehicleType{
        let _sl = "saloon".to_string();
        let _bs = "bus".to_string();
        let _tl = "trailer".to_string();
        if car_type == _sl {
            VehicleType::SALOON
        }else if car_type == _bs{
            VehicleType::BUS
        }else if car_type == _tl{
            VehicleType::TRAILER
        }else{
            VehicleType::OTHER
        }
    }
    // new_car creates an instance of the vehicle type and appends it to the car vector
    pub  fn new_car(&mut self, from_to: String, types: String, reg_no: String) {
        

        for elem in self.car.iter() {
            if elem.reg_no == reg_no {
                env::log_str("Could not create car. Car already exists!");
                return;
            }
        }


        let cr = Vehicle {
            types:self.get_car_type(types),
            reg_no: reg_no,
            from_to: from_to,
        };
        self.car.push(&cr);
    }

    // amount_to_pay returns the toll fees for each trip
    pub fn amount_to_pay(&self, reg_no: String) -> u128 {
        let mut car: Option<Vehicle> = None;

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
                    Some(sm) => sm,
                    None => self.max_fare(),
                }
            }
            None => {
                env::log_str("Pay max fare because car not found");
                self.max_fare()
            }
        };
    }

    // pay deducts the toll fees from the card.
    // if card balance is not sufficient, the user will be prompted to pay with cash
    #[payable]
    pub  fn pay(&mut self, reg_no: String) -> String {
        let amount = self.amount_to_pay(reg_no);

        if env::attached_deposit() < amount {
            env::log_str("Not enough tokens pay with cash");
            return "error".to_string();
        } else {
            Promise::new(env::current_account_id()).transfer(amount);

            return "ok".to_string();
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
    use near_sdk::test_utils::{ VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // new_car should create a new car object and append it to the cars vector
    // The length of cars vector should increase by one for each new car object created
    // The car reg_no must be unique. If car reg_no already exists, no new car should be created
    #[test]
    fn new_car() {
        let mut app = App::default();

        // adding a new car should work
        app.new_car(
            "westlands-mlolongo".to_string(),
            "saloon".to_string(),
            "KDJ 001A".to_string(),
        );
        assert_eq!(app.car.len(), 1);
    }

    // pay should update the account balance by deducting the amount to be paid for each trip
    #[test]
    fn pay() {
        let user = AccountId::new_unchecked("kenn.testnet".to_string());
        let mut _context = get_context(user.clone());
        let bal = one_near() * 20;
        _context.attached_deposit(bal);
        _context.account_balance(bal);

        testing_env!(_context.build());

        let mut app = App::default();

        app.new_car(
            "westlands-mlolongo".to_string(),
            "saloon".to_string(),
            "KDJ 001A".to_string(),
        );

        let res = app.pay("KDJ 001A".to_string());
        assert_eq!(res, "ok".to_string())
    }
}