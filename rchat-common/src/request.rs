use std::fmt::{Display, Formatter, Result};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RequestType {
    login,
    logout,
    msg,
    names,
    help,
}

/*
impl Display for RequestType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &RequestType::login => write!(f, "login"),
            &RequestType::logout => write!(f, "logout"),
            &RequestType::msg => write!(f, "msg"),
            &RequestType::names => write!(f, "names"),
            &RequestType::help => write!(f, "help"),
        }
    }
}
*/

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request {
    pub request: RequestType,
    pub content: Option<String>,
}

#[cfg(test)]mod test {
    extern crate serde_json;

    use super::*;

    #[test]
    fn test1() {
        let reqt1 = RequestType::login;
        let reqt2 = RequestType::msg;
        let reqt1j = serde_json::to_string(&reqt1).unwrap();
        let reqt2j = serde_json::to_string(&reqt2).unwrap();
        let reqt1d: RequestType = serde_json::from_str(&reqt1j).unwrap();
        let reqt2d: RequestType = serde_json::from_str(&reqt2j).unwrap();
        assert_eq!(reqt1, reqt1d);
        assert_eq!(reqt2, reqt2d);
    }

    #[test]
    fn test2() {
        let req1 = Request{request: RequestType::msg, content: Some("hello".to_string())};
        let req2 = Request{request: RequestType::help, content: None};
        let req1j = serde_json::to_string(&req1).unwrap();
        let req2j = serde_json::to_string(&req2).unwrap();
        let req1d: Request = serde_json::from_str(&req1j).unwrap();
        let req2d: Request = serde_json::from_str(&req2j).unwrap();
        assert_eq!(req1, req1d);
        assert_eq!(req2, req2d); 
    } 
}