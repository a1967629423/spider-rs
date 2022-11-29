use pica_client::{
    error::PicaResult,
    picajson::{PicaApiComicsData, PicaApiJSON, PicaApiTokenData},
};
use spider_engine::{
    core::{
        content_fetcher::{BoxContentType, BoxContextID, ContentFetcher},
        content_resolver::ContentType,
    },
    BoxError,
};
pub struct PicaHomeFetcher {
    session: pica_client::session::PicaSession,
    username: String,
    password: String,
    is_login: bool,
}
#[derive(Debug, Clone)]
pub struct PicaHomeContent {
    pub content: PicaApiJSON<PicaApiComicsData>,
}

impl ContentType for PicaHomeContent {
    fn custom_debug(&self) -> Option<Box<dyn std::fmt::Debug>> {
        Some(Box::new(self.clone()))
    }
}

impl PicaHomeFetcher {
    pub fn new(proxy: Option<String>, username: String, password: String) -> Self {
        Self {
            session: pica_client::session::PicaSession::new(proxy),
            username,
            password,
            is_login: false,
        }
    }
    pub async fn login(&mut self) -> PicaResult<PicaApiJSON<PicaApiTokenData>> {
        let res = self
            .session
            .login(self.username.clone(), self.password.clone())
            .await?;
        self.is_login = true;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl ContentFetcher for PicaHomeFetcher {
    type ContentType = BoxContentType;
    type ID = BoxContextID;
    type Error = BoxError;
    async fn fetch_content(&mut self, _id: &Self::ID) -> Result<Self::ContentType, Self::Error> {
        if !self.is_login {
            log::info!("to login");
            let resp = self.login().await.map_err(BoxError::new)?;
            log::info!("login resp {:?}", resp);
        }

        let resp = self
            .session
            .get_comics(pica_client::request::ComicOrder::Default, 0)
            .await
            .map_err(BoxError::new)?;

        Ok(BoxContentType::new(PicaHomeContent { content: resp }))
    }
}
