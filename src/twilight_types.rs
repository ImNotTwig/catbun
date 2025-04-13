use std::sync::{Arc, atomic::AtomicBool};

use http::HeaderMap;
use twilight_http_ratelimiting::InMemoryRatelimiter;
use twilight_model::channel::message::AllowedMentions;

use hyper_util::{
    client::legacy::{Client as HyperClient, connect::HttpConnector},
    rt::TokioExecutor,
};

type HttpsConnector<T> = hyper_tls::HttpsConnector<T>;
type Connector = HttpsConnector<HttpConnector>;

/// So basically, using a non-bot token to interact with the Discord API breaks TOS
/// which means that twilight doesn't allow us to use a user token normally, as they
/// leave the token field private, and force us to use a setter that prepends "Bot "
/// to the token.
/// So, we create a type in our crate with the same layout, to transmute after setting a token of our own choosing.
pub fn new_twilight_http_client(token: String) -> twilight_http::Client {
    let token = Token::new(token);
    unsafe { std::mem::transmute(HttpDiscordClient::new(Some(token))) }
}

#[allow(dead_code)]
#[derive(Default)]
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
