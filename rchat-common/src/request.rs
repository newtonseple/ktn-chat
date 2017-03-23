use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "request")]
pub enum Request {
    login { content: Option<String> },
    logout { content: Option<String> },
    msg { content: Option<String> },
    names { content: Option<String> },
    help { content: Option<String> },
}

impl FromStr for Request {
    type Err = ();

    fn from_str(s: &str) -> Result<Request, ()> {
        let mut words = s.splitn(2, ' ');
        let req = words.nth(0);
        let con = words.map(|s| s.to_string()).nth(0);
        if let Some(request) = req {
            match request {
                "login" => {
                    return Ok(Request::login { content: con });
                },
                "logout" => {
                    return Ok(Request::logout { content: con });
                },
                "msg" => {
                    return Ok(Request::msg { content: con });
                },
                "names" => {
                    return Ok(Request::names { content: con });
                },
                "help" => {
                    return Ok(Request::help { content: con });
                },
                _ => {
                    return Err(());
                },
            };
        }
        Err(())
    }
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
/*
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request {
    pub request: RequestType,
    pub content: Option<String>,
}
*/
#[cfg(test)]
mod test {
    extern crate serde_json;

    use super::*;

    #[test]
    fn test2() {
        let req1 = Request::msg{
            content: Some("hello".to_string()),
        };
        let req2 = Request::help {
            content: None,
        };
        let req1j = serde_json::to_string(&req1).unwrap();
        let req2j = serde_json::to_string(&req2).unwrap();
        let req1d: Request = serde_json::from_str(&req1j).unwrap();
        let req2d: Request = serde_json::from_str(&req2j).unwrap();
        assert_eq!(req1, req1d);
        assert_eq!(req2, req2d);
    }

    #[test]
    fn test3() {
        let msg = "msg Hei på deg";
        let req: Request = msg.parse().expect("parse failed");
        assert_eq!(Request::msg{content: Some("Hei på deg".to_string())}, req);
    }
}