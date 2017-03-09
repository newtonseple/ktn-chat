extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;


#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug, PartialEq)]
enum SomeEnum {
    Foo,
    Bar(i32),
    Baz { a: i32, b: bool },
    Boo { c: SomeStruct },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SomeStruct {
    a: i32,
    b: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum R {
    login,
    logout,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct m {
    request: R,
    content: String,
}

fn main() {
    let a = SomeEnum::Foo;
    let b = SomeEnum::Bar(42);
    let c = SomeEnum::Baz { a: 42, b: true };
    let d = SomeEnum::Boo { c: SomeStruct { a: 24, b: false } };
    let e = m{request: R::login, content: "didrik".to_string()};
    let aj = serde_json::to_string(&a).unwrap();
    let bj = serde_json::to_string(&b).unwrap();
    let cj = serde_json::to_string(&c).unwrap();
    let dj = serde_json::to_string(&d).unwrap();
    let ej = serde_json::to_string(&e).unwrap();
    let ad: Result<SomeEnum, _> = serde_json::from_str(&aj);
    let bd: Result<SomeEnum, _> = serde_json::from_str(&bj);
    let cd: Result<SomeEnum, _> = serde_json::from_str(&cj);
    let dd: Result<SomeEnum, _> = serde_json::from_str(&dj);
    let ed: Result<m, _> = serde_json::from_str(&ej);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);
    println!("e: {:?}", e);
    println!("aj: {:?}", aj);
    println!("bj: {:?}", bj);
    println!("cj: {:?}", cj);
    println!("dj: {:?}", dj);
    println!("ej: {:?}", ej);
    println!("ad: {:?}", ad);
    println!("bd: {:?}", bd);
    println!("cd: {:?}", cd);
    println!("dd: {:?}", dd);
    println!("ed: {:?}", ed);
    assert_eq!(a, ad.unwrap());
    assert_eq!(b, bd.unwrap());
    assert_eq!(c, cd.unwrap());
    assert_eq!(d, dd.unwrap());
}
