use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::common::card_attributes::card_race::card_race_enum::RaceEnum::Dummy;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;
use crate::game_card_support::handler::handler_of_20::game_card_support_20_handler_impl::SupportCard_20_Function;
use crate::game_card_support::handler::handler_of_2::game_card_support_2_handler_impl::SupportCard_2_Function;

use crate::game_card_support::repository::game_card_support_repository::GameCardSupportRepository;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;
use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;


pub struct GameCardSupportRepositoryImpl {
    support_card_functions: HashMap<i32, Box<dyn GameCardSupportHandler>>,
}

struct NoneFunction;

impl GameCardSupportHandler for NoneFunction {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect {
        println!("아직 구현되지 않은 기능입니다.");

        GameCardSupportEffect::new(Dummy, 0)
    }

    unsafe fn use_specific_support_card(&self) -> GameCardSupportEffect {
        println!("아직 구현되지 않은 기능입니다.");

        GameCardSupportEffect::new(Dummy, 0)
    }
}

impl GameCardSupportRepositoryImpl {
    fn new() -> Self {
        let mut support_card_functions = HashMap::new();
        support_card_functions.insert(2, Box::new(SupportCard_2_Function) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(5, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(7, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(10, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(16, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(20, Box::new(SupportCard_20_Function) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(21, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(24, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(28, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(29, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(36, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(41, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(47, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(65, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(69, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(77, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(87, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(94, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(116, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(126, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(143, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(144, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(146, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(156, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(163, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(165, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(166, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(167, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(169, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(170, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(172, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(175, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(186, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(188, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(190, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(195, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);

        GameCardSupportRepositoryImpl { support_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameCardSupportHandler>> {
        self.support_card_functions.get(&number)
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardSupportRepository for GameCardSupportRepositoryImpl {
    unsafe fn call_support_card_repository_table(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect {
        println!("GameCardSupportRepositoryImpl: call_support_card_repository_table()");

        let support_card_id_string = use_support_card_request.get_support_card_number();
        let support_card_id = support_card_id_string.parse::<i32>().unwrap();

        let support_card_execution_handler = self.support_card_functions.get(&support_card_id);
        support_card_execution_handler.unwrap().use_support_card(use_support_card_request)
    }

    unsafe fn call_support_card_repository_handler(&self, support_card_id: i32) -> GameCardSupportEffect {
        println!("GameCardSupportRepositoryImpl: call_support_card_repository_handler()");

        let support_card_execution_handler = self.support_card_functions.get(&support_card_id);
        support_card_execution_handler.unwrap().use_specific_support_card()
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use crate::common::card_attributes::card_race::card_race_enum::RaceEnum::{Human, Undead};
    use super::*;
    use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

    #[test]
    fn test_game_card_support_repository_impl() {
        let repository = GameCardSupportRepositoryImpl::new();

        let number1 = 2;
        let function1 = repository.get_function(number1);
        assert!(function1.is_some());

        let use_support_card_request = UseSupportCardRequest::new("test".parse().unwrap(), "6".parse().unwrap(), "2".parse().unwrap());

        let response1 = unsafe { function1.unwrap().use_support_card(use_support_card_request) };
        let energy_from_deck = response1.get_energy_from_deck();
        let energy_count = energy_from_deck.get_energy_count();
        let race = energy_from_deck.get_race();
        assert_eq!(response1.get_energy_from_deck().get_energy_count(), 2);
        assert_eq!(response1.get_energy_from_deck().get_race(), &Undead);

        let number2 = 93;
        let function2 = repository.get_function(number2);
        assert!(function2.is_none());
    }

    #[test]
    fn test_none_function() {
        let mut output = Vec::new();
        let mut capture = io::Cursor::new(&mut output);
        writeln!(capture, "아직 구현되지 않은 기능입니다.").unwrap();

        let none_function = NoneFunction;
        let request = UseSupportCardRequest::new("test".parse().unwrap(), "6".parse().unwrap(), "2".parse().unwrap());;
        unsafe { none_function.use_support_card(request); }

        let captured_output = String::from_utf8(output.clone()).unwrap();
        assert_eq!(captured_output.trim(), "아직 구현되지 않은 기능입니다.");
    }
}

