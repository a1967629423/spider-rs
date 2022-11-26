use spider_engine::core::{
    content_fetcher::{BoxContentType, BoxContextID, BoxError, ContentFetcher},
    content_resolver::ContentType,
};

#[derive(Debug, Clone)]
pub struct BaiduHotSearchContent {
    pub contents: Vec<String>,
}
impl ContentType for BaiduHotSearchContent {
    fn custom_debug(&self) -> Option<Box<dyn std::fmt::Debug>> {
        Some(Box::new(self.clone()))
    }
}
pub struct BaiduHotSearchFetcher {}
#[async_trait::async_trait]
impl ContentFetcher for BaiduHotSearchFetcher {
    type ContentType = BoxContentType;
    type Error = BoxError;
    type ID = BoxContextID;
    async fn fetch_content(&mut self, _id: &Self::ID) -> Result<Self::ContentType, Self::Error> {
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.append("User-Agent",reqwest::header::HeaderValue::from_static( "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36 Edg/107.0.1418.52"));
            headers
        };
        let result = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(BoxError::new)?
            .get("https://top.baidu.com/board?platform=pc&sa=pcindex_entry")
            .send()
            .await
            .map_err(BoxError::new)?
            .text()
            .await
            .map_err(BoxError::new)?;
        let res = visdom::Vis::load(result).map_err(BoxError::from_send_sync_box)?;
        let elements = res.find(".item-wrap_2oCLZ .c-single-text-ellipsis");

        let contents = elements.map(|_idx, el| el.text());

        Ok(BoxContentType::new(BaiduHotSearchContent { contents }))
    }
}
