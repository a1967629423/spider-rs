use std::{collections::HashMap, str::FromStr};

use bytes::{Bytes, BytesMut, BufMut};
#[derive(Debug,Clone, Copy,PartialEq, Eq,PartialOrd, Ord)]
pub enum ComicOrder {
    Default,
    Newest,
    Oldest,
    Point,
    Love
}
impl ToString for ComicOrder {
    fn to_string(&self) -> String {
        match self {
            ComicOrder::Default => {
                "ua".into()
            },
            ComicOrder::Newest => {
                "dd".into()
            },
            ComicOrder::Oldest => {
                "da".into()
            },
            ComicOrder::Point => {
                "vd".into()
            },
            ComicOrder::Love => {
                "ld".into()
            }
        }
    }
}
#[derive(Debug,Clone, Copy,PartialEq, Eq,PartialOrd, Ord)]
pub enum PicaRequestMethod {
    GET,
    POST
}
impl PicaRequestMethod {
    pub fn to_str(&self) ->&'static str {
        match self {
            PicaRequestMethod::GET => {
                "GET"
            },
            PicaRequestMethod::POST => {
                "POST"
            }
        }
    }
}
impl From<PicaRequestMethod> for reqwest::Method {
    fn from(p: PicaRequestMethod) -> Self {
        match p {
            PicaRequestMethod::GET => {
                Self::GET
            },
            PicaRequestMethod::POST => {
                Self::POST
            }
        }
    }
}
#[derive(Debug,Clone)]
pub struct PicaRequest {
    pub headers:HashMap<String,String>,
    pub query_url:url::Url,
    pub method:PicaRequestMethod,
    pub body:Bytes,
    pub order:ComicOrder
}
impl Default for PicaRequest {
    fn default() -> Self {
        Self::new()
    }
}
impl PicaRequest {
    pub fn new() -> Self {
        Self {
            headers: {
                let mut h = HashMap::new();
                h.insert("accept".into(), "application/vnd.picacomic.com.v1+json".into());
                h.insert("app-channel".into(), "1".into());
                h.insert("app-uuid".into(),"defaultUuid".into());
                h.insert("app-platform".into(), "android".into());
                h.insert("app-version".into(), "2.2.1.2.3.3".into());
                h.insert("app-build-version".into(), "44".into());
                h.insert("User-Agent".into(), "okhttp/3.8.1".into());
                h.insert("image-quality".into(), "original".into()); // 默认原图
                h
            },
            query_url:url::Url::from_str("https://picaapi.picacomic.com").unwrap(),
            method:PicaRequestMethod::GET,
            body:Bytes::new(),
            order:ComicOrder::Default
        }
    }
    pub fn add_header(&mut self,key:String,value:String) {
        self.headers.insert(key, value);
    }
    pub fn set_body(&mut self,body:Bytes) {
        self.body = body;
    }
    pub fn set_order(&mut self,order:ComicOrder) {
        self.order = order;
    }
    pub fn set_page(&mut self,mut page:usize) {
        page +=  1;
        self.query_url.query_pairs_mut().append_pair("page", page.to_string().as_str());
    }
    pub fn set_url_param(&mut self,key:&str,value:&str) {
        self.query_url.query_pairs_mut().append_pair(key, value);
    }
    pub fn set_path(&mut self,path:&str) {
        self.query_url.set_path(path);
    }
    pub fn set_host(&mut self,host:&str) {
        self.query_url.set_host(Some(host)).unwrap();
    }
    pub fn set_method(&mut self,method:PicaRequestMethod) {
        self.method = method;
    }
    pub fn set_json_body<T>(&mut self,data:&T) where T:serde::Serialize {
        let data = Bytes::from_iter(serde_json::to_vec(data).unwrap());
        self.set_body(data);

    }
    pub fn sign(&mut self) {
        self.sign_with_ts(time::OffsetDateTime::now_utc().unix_timestamp());
    }
    fn sign_with_ts(&mut self,ts:i64) {
        use sha2::Sha256;
        use hex::encode;
        use hmac::{Hmac,Mac};
        type HmacSha256 = Hmac<Sha256>;
        let ts = ts;
        const NONCE:&str = "b1ab87b4800d4d4590a11701b8551afa";
        const API_KEY:&str = "C69BAF41DA5ABD1FFEDC6D2FEA56B";
        const SECRET:&str = "~d}$Q7$eIni=V)9\\RK/P.RM4;9[7|@/CA}b~OW!3?EV`:<>M7pddUBL5n|0/*Cn";
        self.add_header("time".into(), ts.to_string());
        self.add_header("nonce".into(), NONCE.into());
        self.add_header("api-key".into(), API_KEY.into());
        let mut builder = BytesMut::new();
        builder.put(self.query_url.path().to_lowercase().as_bytes());
        if let Some(query) = self.query_url.query() {
            builder.put("?".as_bytes());
            builder.put(query.to_lowercase().as_bytes());
        }
        builder.put(ts.to_string().to_lowercase().as_bytes());
        builder.put(NONCE.to_lowercase().as_bytes());
        builder.put(self.method.to_str().to_lowercase().as_bytes());
        builder.put(API_KEY.to_lowercase().as_bytes());
        let final_raw = &builder[1..];
        
        let mut hasher = HmacSha256::new_from_slice(SECRET.as_bytes()).expect("HMAC can take key of any size");
        hasher.update(final_raw);
        let hash = hasher.finalize();

        self.add_header("signature".into(),encode(hash.into_bytes()));
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use serde::{Serialize, Deserialize};
    #[test]
    pub fn test_sign() {
        #[derive(Debug,Serialize,Deserialize)]
        struct LoginReq {
            email:String,
            password:String
        }
        let mut req = PicaRequest::new();
        req.set_path("/auth/sign-in");
        req.add_header("Content-Type".into(), "application/json; charset=UTF-8".into());
        req.set_json_body(&LoginReq {
            email:"test@qq.com".into(),
            password:"123456789".into()
        } );
        req.sign_with_ts(1669544553);
        assert_eq!(req.headers.get("signature").unwrap(),"5bcf7e2355121582b7c8027ea15030d37b492c6c9f99336f4cd8f7d9d1d271a1")

    }
}