use std::{env, num::NonZeroU64, sync::Arc};

use twilight_cache_inmemory::{DefaultInMemoryCache, InMemoryCache};
use twilight_gateway::{Intents, ShardId};
use twilight_http::Client as HttpClient;

use crate::twilight_types;

use iced::{
    Element, Length, Task,
    widget::{button, column, container, row, text},
};

#[derive(Debug, Clone)]
pub enum Message {
    None(()),
    Connect(()),
}

pub type State = Arc<StateRef>;

pub struct StateRef {
    http: HttpClient,
    cache: InMemoryCache,
}

async fn connected(state: State) {
    let name = state
        .http
        .current_user()
        .into_future()
        .await
        .unwrap()
        .model()
        .await
        .unwrap()
        .name;

    tracing::info!("Logged in as `{name}`.");
}

impl StateRef {
    fn container_style() -> container::Style {
        container::Style::default()
            .background(iced::Color::from_rgb(
                0x0F as f32 / 255.0,
                0x10 as f32 / 255.0,
                0x10 as f32 / 255.0,
            ))
            .color(iced::Color::from_rgb(
                0xC9 as f32 / 255.0,
                0xC7 as f32 / 255.0,
                0xCD as f32 / 255.0,
            ))
    }

    fn channels_view(&self) -> Element<Message> {
        container(
            text("channels")
                .center()
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fixed(250.0))
        .style(|_| Self::container_style())
        .into()
    }
    fn servers_view(&self) -> Element<Message> {
        container(
            text("servers")
                .center()
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fixed(75.0))
        .style(move |_| Self::container_style())
        .into()
    }
    fn messages_view(&self) -> Element<Message> {
        container(
            text("chat")
                .center()
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .style(move |_| Self::container_style())
        .into()
    }
    fn members_view(&self) -> Element<Message> {
        container(
            text("server members")
                .center()
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fixed(250.0))
        .style(move |_| Self::container_style())
        .into()
    }
    fn input_view(&self) -> Element<Message> {
        container(
            text("text input")
                .width(Length::Fill)
                .height(Length::Fill)
                .center(),
        )
        .height(75.0)
        .style(move |_| Self::container_style())
        .into()
    }
}

impl Default for StateRef {
    fn default() -> Self {
        let token = env::var("TOKEN")
            .expect("Define the TOKEN environmental variable before running catbun.");

        let http_client = twilight_types::new_twilight_http_client(token.clone());
        let cache = DefaultInMemoryCache::builder()
            .message_cache_size(10)
            .build();

        // let mut shard =
        //     twilight_types::new_twilight_gateway_client(ShardId::ONE, token, Intents::all());

        // for message in http_client
        //     .channel_messages(twilight_model::id::Id::from(
        //         NonZeroU64::new(909989219278159932).unwrap(),
        //     ))
        //     .await?
        //     .model()
        //     .await?
        // {
        //     tracing::info!("Message: {}", message.content);
        // }

        StateRef {
            http: http_client,
            cache,
        }
    }
}

pub trait AppHandler {
    // required functions for Iced
    fn update(&mut self, message: Message) -> Task<Message>;
    fn view(&self) -> Element<Message>;
}

impl AppHandler for State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None(_) => Task::none(),
            Message::Connect(_) => Task::perform(connected(Arc::clone(&self)), Message::None),
        }
    }

    fn view(&self) -> Element<Message> {
        //TODO: Mockup a layout for a server
        row![
            self.servers_view(),
            self.channels_view(),
            column![self.messages_view(), self.input_view()].spacing(10),
            self.members_view()
        ]
        .spacing(10)
        .into()
    }
}
