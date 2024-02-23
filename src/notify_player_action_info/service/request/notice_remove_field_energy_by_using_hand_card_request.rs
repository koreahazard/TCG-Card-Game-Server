#[derive(Debug)]
pub struct NoticeRemoveFieldEnergyByUsingHandCardRequest {
    account_unique_id: i32,
    opponent_unique_id: i32,
    used_hand_card_id: i32,
}

impl NoticeRemoveFieldEnergyByUsingHandCardRequest {
    pub fn new(account_unique_id: i32,
               opponent_unique_id: i32,
               used_hand_card_id: i32) -> Self {
        NoticeRemoveFieldEnergyByUsingHandCardRequest {
            account_unique_id,
            opponent_unique_id,
            used_hand_card_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }
}