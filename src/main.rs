use std::sync::Arc;

use twilight_http::Client as HttpClient;

use iced::Task;

use state::{AppHandler, Message, State};

mod state;
mod twilight_types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = iced::application("Catbun", State::update, State::view)
        .theme(|_state| iced::theme::Theme::Oxocarbon);

    Ok(app.run_with(|| -> (State, Task<Message>) {
        (
            State::default(),
            Task::perform((async || {})(), Message::Connect),
        )
    })?)
}
