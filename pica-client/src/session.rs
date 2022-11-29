use crate::{
    client::PicaClient,
    error::PicaResult,
    picajson::{PicaApiComicsData, PicaApiJSON, PicaApiTokenData, PicaApiCategoryData, PicaApiEpsData, PicaApiComicData, PicaThumb},
    request::{ComicOrder, PicaRequest, PicaRequestMethod},
};
use serde::{Deserialize, Serialize};
pub struct PicaSession {
    token: Option<String>,
    client: PicaClient,
}

impl PicaSession {
    pub fn new(proxy: Option<String>) -> Self {
        Self {
            token: None,
            client: PicaClient::new(proxy),
        }
    }
    fn new_request_with_order(&self,order:ComicOrder) -> PicaRequest {
        let mut req = self.new_request::<true>();
        req.add_header("s".into(), order.to_string());
        req
    }
    fn new_request<const WITH_API:bool>(&self) -> PicaRequest {
        let mut req = PicaRequest::new();
        if let Some(token) = &self.token {
            req.add_header("authorization".into(), token.clone());
        }
        if WITH_API {
            req.add_header(
                "Content-Type".into(),
                "application/json; charset=UTF-8".into(),
            );
        }
        req
    }
}

impl PicaSession {
    /// 登录并设置Session
    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> PicaResult<PicaApiJSON<PicaApiTokenData>> {
        #[derive(Debug, Serialize, Deserialize)]
        struct LoginReq {
            email: String,
            password: String,
        }
        let mut req = self.new_request::<true>();
        req.set_method(PicaRequestMethod::POST);
        req.set_path("/auth/sign-in");

        req.set_json_body(&LoginReq {
            email: username,
            password,
        });
        let resp = self
            .client
            .send(req)
            .await?
            .json::<PicaApiJSON<PicaApiTokenData>>()
            .await?;
        self.token.replace(resp.data.token.clone());
        Ok(resp)
    }
    /// 搜索
    pub async fn search(
        &self,
        keyword: String,
        order: ComicOrder,
        page: usize,
    ) -> PicaResult<PicaApiJSON<PicaApiComicsData>> {
        #[derive(Debug, Serialize, Deserialize)]
        struct SearchReq {
            keyword: String,
            sort: String,
        }
        let mut req = self.new_request::<true>();
        req.set_path("/comics/advanced-search".into());
        req.set_method(PicaRequestMethod::POST);

        req.set_json_body(&SearchReq {
            keyword,
            sort: order.to_string(),
        });
        req.set_page(page);

        Ok(self
            .client
            .send(req)
            .await?
            .json::<PicaApiJSON<PicaApiComicsData>>()
            .await?)
    }
    /// 获取分区内容
    pub async fn get_block(&self,block_name:String,order: ComicOrder,page: usize) -> PicaResult<PicaApiJSON<PicaApiComicsData>> {
        let mut req = self.new_request_with_order(order);
        req.set_method(PicaRequestMethod::GET);
        req.set_path("/comics");
        req.set_url_param("c", &block_name);
        req.set_page(page);
        Ok(self.client.send(req).await?.json::<PicaApiJSON<PicaApiComicsData>>().await?)
    }

    /// 获取主页目录
    pub async fn get_category(&self) -> PicaResult<PicaApiJSON<PicaApiCategoryData>> {
        let mut req = self.new_request::<true>();
        req.set_method(PicaRequestMethod::GET);
        req.set_path("/categories");
        Ok(self.client.send(req).await?.json::<PicaApiJSON<PicaApiCategoryData>>().await?)
    }

    /// 获取漫画分集
    pub async fn get_eps(&self,id:String,page:usize) -> PicaResult<PicaApiJSON<PicaApiEpsData>>{
        let mut req = self.new_request::<true>();
        req.set_method(PicaRequestMethod::GET);
        req.set_path(format!("/comics/{}/eps",id).as_str());
        req.set_page(page);

        Ok(self.client.send(req).await?.json::<PicaApiJSON<PicaApiEpsData>>().await?)
    }

    /// 获取所有漫画列表
    pub async fn get_comics(&self,order: ComicOrder,page:usize) -> PicaResult<PicaApiJSON<PicaApiComicsData>> {
        let mut req = self.new_request_with_order(order);
        req.set_method(PicaRequestMethod::GET);
        req.set_path("/comics");
        req.set_page(page);
        Ok(self.client.send(req).await?.json::<PicaApiJSON<PicaApiComicsData>>().await?)
    }
    /// 获取漫画详情
    pub async fn get_comics_info(&self,id:String) -> PicaResult<PicaApiJSON<PicaApiComicData>> {
        let mut req = self.new_request::<true>();
        req.set_method(PicaRequestMethod::GET);
        req.set_path(format!("/comics/{}",id).as_str());
        Ok(self.client.send(req).await?.json::<PicaApiJSON<PicaApiComicData>>().await?)
    }

    /// 获取图片数据
    pub async fn fetch_media(&self,thumb:PicaThumb) ->PicaResult<bytes::Bytes> {
        let mut req = self.new_request::<false>();
        req.set_method(PicaRequestMethod::GET);
        req.set_path(format!("/static/{}",thumb.path).as_str());
        req.set_host(&thumb.file_server);

        Ok(self.client.send(req).await?.bytes().await?)
    }
}
