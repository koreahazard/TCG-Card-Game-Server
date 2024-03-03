use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormBasicAttackToMainCharacter {
    player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
}

impl NotifyFormBasicAttackToMainCharacter {
    pub fn new(
        player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> Self {

        NotifyFormBasicAttackToMainCharacter {
            player_main_character_health_point_map,
            player_main_character_survival_map,
        }
    }
}