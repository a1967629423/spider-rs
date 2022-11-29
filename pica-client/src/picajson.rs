use serde::{Serialize,Deserialize};
/// 每个分区的信息
#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PicaCategory {
    #[serde(default="Default::default")]
    pub active:bool,
    #[serde(default="Default::default")]
    pub is_web:bool,
    pub link:Option<String>,
    pub title:String,
    pub thumb:PicaThumb
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaImage {
    pub id:String,
    pub media:PicaThumb,
}
// 获取comic时会返回的简单数据
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaEp {
    pub id:String,
    pub order:usize,
    pub title:String,
    pub updated_at:String,
}
#[derive(Debug,Clone,Default,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PicaThumb {
    pub file_server:String,
    pub path:String,
    pub original_name:Option<String>,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PicaComic {
    #[serde(default="Default::default")]
    pub author:String,
    pub categories:Vec<String>,
    #[serde(default="Default::default")]
    pub eps_count:usize,
    #[serde(default="Default::default")]
    pub finished:bool,
    #[serde(default="Default::default")]
    pub likes_count:usize,
    #[serde(default="Default::default")]
    pub pages_count:usize,
    #[serde(default="Default::default")]
    pub thumb:PicaThumb,
    pub title:String,
    #[serde(rename="_id")]
    pub id:String,
}
#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct PicaUser {
    #[serde(default="Default::default")]
    pub avatar:PicaThumb,
    #[serde(default="Default::default")]
    pub character:String,
    #[serde(default="Default::default")]
    pub characters:Vec<String>,
    #[serde(default="Default::default")]
    pub exp:usize,
    #[serde(default="Default::default")]
    pub gender:String,
    #[serde(default="Default::default")]
    pub level:isize,
    #[serde(default="Default::default")]
    pub name:String,
    #[serde(default="Default::default")]
    pub role:String,
    #[serde(default="Default::default")]
    pub slogan:String,
    #[serde(default="Default::default")]
    pub title:String,
    #[serde(default="Default::default")]
    pub verified:bool,
    #[serde(rename="_id")]
    pub id:String,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PicaComicDetail {
    #[serde(default="Default::default")]
    pub allow_comment:bool,
    #[serde(default="Default::default")]
    pub allow_download:bool,
    #[serde(default="Default::default")]
    pub author:String,
    #[serde(default="Default::default")]
    pub categories:Vec<String>,
    #[serde(default="Default::default")]
    pub chinese_team:String,
    #[serde(default="Default::default")]
    pub comments_count:String,
    #[serde(default="Default::default")]
    pub created_at:String,
    #[serde(default="Default::default")]
    pub description:String,
    #[serde(default="Default::default")]
    pub eps_count:usize,
    #[serde(default="Default::default")]
    pub finished:bool,
    #[serde(default="Default::default")]
    pub is_favourite:bool,
    #[serde(default="Default::default")]
    pub is_liked:bool,
    #[serde(default="Default::default")]
    pub likes_count:usize,
    #[serde(default="Default::default")]
    pub pages_count:usize,
    #[serde(default="Default::default")]
    pub tags:Vec<String>,
    #[serde(default="Default::default")]
    pub thumb:PicaThumb,
    #[serde(default="Default::default")]
    pub title:String,
    #[serde(default="Default::default")]
    pub total_likes:usize,
    #[serde(default="Default::default")]
    pub total_views:usize,
    #[serde(default="Default::default")]
    pub updated_at:usize,
    #[serde(default="Default::default")]
    pub creator:PicaUser,
    #[serde(rename="_id")]
    pub id:String,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct  PicaPaginationInfo {
    #[serde(default="Default::default")]
    pub limit:usize,
    #[serde(default="Default::default")]
    pub page:usize,
    #[serde(default="Default::default")]
    pub pages:usize,
    pub total:usize,
}



#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiJSON<T> {
    pub code:isize,
    pub message:String,
    pub data:T,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiTokenData {
    pub token:String,
}
/// 首页目录
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiCategoryData {
    pub categories:Vec<PicaCategory>
}

/// 获取一部动漫详情
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiComicData {
    comic:PicaComicDetail
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiComicsDataInner {
    pub docs:Vec<PicaComic>,
    #[serde(flatten)]
    pub page_info:PicaPaginationInfo
}
/// 分区或者搜索是返回的数据格式
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiComicsData {
    pub comics:PicaApiComicsDataInner
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiEpsDataInner {
    pub docs:Vec<PicaEp>,
    #[serde(flatten)]
    pub page_info:PicaPaginationInfo
}
/// 获取漫画分集
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiEpsData {
    pub eps:PicaApiEpsDataInner
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaAPiPagesDataInner {
    pub docs:Vec<PicaImage>,
    #[serde(flatten)]
    pub page_info:PicaPaginationInfo
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PicaApiPagesData {
    pub pages:PicaAPiPagesDataInner
}