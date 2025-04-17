use std::sync::{Arc, atomic::AtomicBool};

use http::HeaderMap;
use tokio_websockets::Connector as TokioConnector;
use twilight_gateway::{Intents, Session, Shard, ShardId, queue::InMemoryQueue};
use twilight_http_ratelimiting::InMemoryRatelimiter;

use twilight_model::{
    channel::message::AllowedMentions,
    gateway::payload::outgoing::{
        identify::IdentifyProperties, update_presence::UpdatePresencePayload,
    },
};

use hyper_util::{
    client::legacy::{Client as HyperClient, connect::HttpConnector},
    rt::TokioExecutor,
};

/// So basically, using a non-bot token to interact with the Discord API breaks TOS
/// which means that twilight doesn't allow us to use a user token normally, as they
/// leave the token field private, and force us to use a setter that prepends "Bot "
/// to the token.
/// So, we create a type in our crate with the same layout, to transmute after setting a token of our own choosing.
pub fn new_twilight_http_client(token: String) -> twilight_http::Client {
    let token = Token::new(token);
    unsafe { std::mem::transmute(HttpDiscordClient::new(Some(token))) }
}

/// this function uses transmute for the same reason `new_twilight_http_client` does
pub fn new_twilight_gateway_client(id: ShardId, token: String, intents: Intents) -> Shard {
    Shard::with_config(id, unsafe {
        std::mem::transmute(DiscordGatewayConfig::new(intents, token))
    })
}

#[allow(dead_code)]
struct Token {
    inner: Box<str>,
}

impl Token {
    fn new(token: String) -> Self {
        Self {
            inner: token.into_boxed_str(),
        }
    }
}

type HttpsConnector<T> = hyper_tls::HttpsConnector<T>;
type Connector = HttpsConnector<HttpConnector>;

#[allow(dead_code)]
struct HttpDiscordClient {
    default_allowed_mentions: Option<AllowedMentions>,
    default_headers: Option<HeaderMap>,
    http: HyperClient<Connector, http_body_util::Full<bytes::Bytes>>,
    proxy: Option<Box<str>>,
    ratelimiter: Option<Box<dyn twilight_http_ratelimiting::Ratelimiter>>,
    timeout: std::time::Duration,

    token_invalidated: Option<Arc<AtomicBool>>,
    token: Option<Token>,
    use_http: bool,
}

impl HttpDiscordClient {
    fn new(token: Option<Token>) -> Self {
        let mut connector = HttpConnector::new();
        connector.enforce_http(false);

        let connector = hyper_tls::HttpsConnector::new_with_connector(connector);

        let http =
            hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(connector);

        let token_invalidated = Some(Arc::new(AtomicBool::new(false)));

        Self {
            http,
            default_headers: None,
            proxy: None,
            ratelimiter: Some(Box::new(InMemoryRatelimiter::default())),
            timeout: std::time::Duration::from_secs(10),
            token_invalidated,
            token,
            default_allowed_mentions: None,
            use_http: false,
        }
    }
}

#[allow(dead_code)]
struct DiscordGatewayConfig<Q = InMemoryQueue> {
    identify_properties: Option<IdentifyProperties>,
    intents: Intents,
    large_threshold: u64,
    presence: Option<UpdatePresencePayload>,
    proxy_url: Option<Box<str>>,
    queue: Q,
    ratelimit_messages: bool,
    resume_url: Option<Box<str>>,
    session: Option<Session>,
    tls: Arc<TokioConnector>,
    token: Token,
}

impl DiscordGatewayConfig {
    fn new(intents: Intents, token: String) -> Self {
        Self {
            identify_properties: None,
            intents,
            large_threshold: 50,
            presence: None,
            proxy_url: None,
            queue: InMemoryQueue::default(),
            ratelimit_messages: true,
            resume_url: None,
            session: None,
            tls: Arc::new(TokioConnector::new().unwrap()),
            token: Token::new(token),
        }
    }
}
