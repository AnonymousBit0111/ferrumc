use ferrumc_ecs::errors::ECSError;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::ConnectionState;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::chat::PlayerChatEvent;
use ferrumc_net::utils::ecs_helpers::EntityExt;
use ferrumc_net::GlobalState;
use tracing::{error, info, trace, warn};

#[event_handler]
async fn handle_chat(
    chat_event: PlayerChatEvent,
    state: GlobalState,
) -> Result<PlayerChatEvent, NetError> {
    trace!("Handling player chat event");

    // set connection state to handshake
    let entity = chat_event.entity;
    let Ok(connection_state) = entity.get::<ConnectionState>(state) else {
        error!("Failed to get connection state");
        return Err(NetError::ECSError(ECSError::ComponentNotFound));
    };
    match *connection_state {
        ConnectionState::Handshaking => warn!("a Player tried to send a chat while handshaking!"),
        ConnectionState::Status => {
            warn!("a Client tried to send a chat while in the status state!")
        }

        ConnectionState::Login => warn!("a Client tried to send a chat while in the login state!"),
        ConnectionState::Play => {}
        ConnectionState::Configuration => {
            warn!("a Client tried to send a chat while in the config state!")
        }
    };
    info!("Chat message: {}", chat_event.message.message);

    Ok(chat_event)
}
