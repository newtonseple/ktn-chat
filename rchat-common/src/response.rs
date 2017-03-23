use std::fmt;
use std::fmt::{Display, Formatter};
use std::vec::Vec;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "response")]
pub enum Response {
    error {
        timestamp: String,
        sender: String,
        content: String,
    },
    info {
        timestamp: String,
        sender: String,
        content: String,
    },
    message {
        timestamp: String,
        sender: String,
        content: String,
    },
    history {
        timestamp: String,
        sender: String,
        content: Vec<Response>,
    },
}


impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Response::error { ref timestamp, ref sender, ref content } => {
                write!(f, "[{}, {}] {}: {}", timestamp, sender, "error", content)
            }
            &Response::info { ref timestamp, ref sender, ref content } => unimplemented!(),
            &Response::message { ref timestamp, ref sender, ref content } => unimplemented!(),
            &Response::history { ref timestamp, ref sender, ref content } => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate serde_json;

    use super::*;

    #[test]
    fn serde_test_simple() {
        let a = Response::error {
            timestamp: "I dag".to_string(),
            sender: "meg".to_string(),
            content: "FAIL!!!".to_string(),
        };
        let b = Response::info {
            timestamp: "2017-17-3 15:51".to_string(),
            sender: "deg".to_string(),
            content: "dette er informasjon".to_string(),
        };
        let c = Response::message {
            timestamp: "18234701239847".to_string(),
            sender: "".to_string(),
            content: "hei".to_string(),
        };

        let aj = serde_json::to_string(&a).unwrap();
        let bj = serde_json::to_string(&b).unwrap();
        let cj = serde_json::to_string(&c).unwrap();

        let ad: Response = serde_json::from_str(&aj).unwrap();
        let bd: Response = serde_json::from_str(&bj).unwrap();
        let cd: Response = serde_json::from_str(&cj).unwrap();

        assert_eq!(a, ad);
        assert_eq!(b, bd);
        assert_eq!(c, cd);
    }

    #[test]
    fn serde_test_hard() {
        let a = Response::history {
            timestamp: "a".to_string(),
            sender: "b".to_string(),
            content: vec![Response::message{timestamp: "a1".to_string(), 
                sender: "b1".to_string(), 
                content: "c1".to_string()}; 10],
        };
        let aj = serde_json::to_string(&a).unwrap();
        let ad = serde_json::from_str(&aj).unwrap();
        assert_eq!(a, ad);
    }
}