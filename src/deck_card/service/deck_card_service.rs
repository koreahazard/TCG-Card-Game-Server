use async_trait::async_trait;

use crate::deck_card::service::request::deck_card_list_request::DeckCardListRequest;
use crate::deck_card::service::response::deck_card_list_response::DeckCardListResponse;
use crate::deck_card::service::request::deck_configuration_request::DeckConfigurationRequest;
use crate::deck_card::service::response::deck_configuration_response::DeckConfigurationResponse;

#[async_trait]
pub trait DeckCardService {
    async fn deck_configuration_register(&self, deck_configuration_request: DeckConfigurationRequest) -> DeckConfigurationResponse;
    async fn deck_card_list(&self, deck_card_list_request: DeckCardListRequest) -> DeckCardListResponse;
}