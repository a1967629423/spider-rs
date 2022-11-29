use std::{str::FromStr, future::Future};

use crate::request::PicaRequest;

pub struct PicaClient {
    client: reqwest::Client,
}

impl PicaClient {
    pub fn new(proxy: Option<String>) -> Self {
        let mut client_builder = reqwest::ClientBuilder::new();
        if let Some(proxy) = proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy).unwrap());
        }
        PicaClient {
            client: client_builder.build().unwrap(),
        }
    }

    pub fn send(&self, mut req: PicaRequest) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        req.sign();
        self.client
            .request(req.method.into(), req.query_url)
            .body(req.body)
            .headers(
                req.headers
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            reqwest::header::HeaderName::from_str(&k).unwrap(),
                            reqwest::header::HeaderValue::from_str(&v).unwrap(),
                        )
                    })
                    .collect(),
            ).send()
    }
}
