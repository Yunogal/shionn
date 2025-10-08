// https://api.vndb.org/kana#introduction
// https://api.vndb.org/kana#usage-terms

use std::fs::File;
use std::io::{self, BufWriter, Write};

use serde::{Deserialize, Serialize};

use ureq::{self, Body};
use ureq::{Agent, Proxy};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub more: bool,
    pub results: Box<[Result]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    pub alttitle: Option<Box<str>>,
    pub id: Box<str>,
    pub released: Box<str>,
    pub title: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response2 {
    pub more: bool,
    pub results: Box<[Result2]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result2 {
    pub aliases: Box<[Box<str>]>,
    pub extlinks: Box<[Extlink]>,

    pub id: Box<str>,
    pub name: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extlink {
    pub label: Box<str>,
    pub url: Box<str>,
}
const SOFT: &'static str = "";

fn post(agent: &Agent, url: &str, request: &str) -> Body {
    let response = agent
        .post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .send(request.as_bytes()).unwrap();
    if response.status() != 200 {
        println!("{}", response.status());
    }
    response.into_body()
}
fn main() -> io::Result<()> {
    // let proxy = Proxy::new("http://127.0.0.1:1116").unwrap();
    let config = Agent::config_builder()
        .https_only(true)
    //  .proxy(Some(proxy))
        .build();
    let agent = config.new_agent();
    let soft = format!(
        r#"{{
    "filters": ["id", "=", "{}"],
    "fields": "id,name,aliases,extlinks{{url,label}}",
    "results":1
}}"#,
        SOFT
    );

    let body = post(
        &agent,
        "https://api.vndb.org/kana/producer",
        &soft,
    );
    let json = body.into_reader();
    let Response2 { more, results } = serde_json::from_reader(json).unwrap();
    assert!(!more);
    assert!(results.len() == 1);
    let Result2 {
        aliases,
        extlinks,
        id,
        name,
    } = &results[0];

    let file = File::create(name.as_ref())?;
    let mut buf = BufWriter::new(file);

    let format = format!("{} {} ", id, name);
    buf.write(format.as_bytes())?;
    for aliase in aliases {
        let format = format!("{} ", aliase);
        buf.write(format.as_bytes())?;
    }
    buf.write(b"\n")?;
    for Extlink { label, url } in extlinks {
        let format = format!("\n[{}]({})\n", label, url);
        buf.write(format.as_bytes())?;
    }

    let galgame = format!(
        r#"{{
    "filters": ["developer", "=", ["id","=","{}"]],
    "fields": "id,title,alttitle,released",
    "sort":"released",
    "results":100
}}"#,
        SOFT
    );
    let body = post(&agent, "https://api.vndb.org/kana/vn", &galgame);
    let json = body.into_reader();
    let Response { more, results } = serde_json::from_reader(json).unwrap();
    assert!(!more);
    for Result {
        alttitle,
        id,
        released,
        title,
    } in results
    {
        let format = format!(
            "\n{id:<8} {released}\n{title}\n{}\n",
            alttitle.unwrap_or("Why don't I have an alttitle?".into())
        );
        buf.write(format.as_bytes())?;
    }
    Ok(())
}
