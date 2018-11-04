#![deny(warnings)]
extern crate futures;

extern crate hyper;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use hyper::rt::{self, Future, Stream};
use hyper::{Body, Client, Error, Response};

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    #[serde(rename = "userId")]
    user_id: i64,
    id: i64,
    title: String,
    body: String,
}

fn fetch_url(url: hyper::Uri) -> impl Future<Item = (), Error = ()> {
    let client = Client::new();

    client
        .get(url)
        .then(stringify)
        .map(parse_json)
        .map(get_unique_user_ids)
        .inspect(log)
        .map(|_| ())
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

fn get_unique_user_ids(posts: Vec<Post>) -> Vec<i64> {
    let mut all_user_ids: Vec<i64> = posts.into_iter().map(|post| post.user_id).collect();
    all_user_ids.sort();
    all_user_ids.dedup();
    all_user_ids
}

fn log(anything: &impl std::fmt::Debug) {
    println!("{:#?}", anything);
}

fn parse_json(data: String) -> Vec<Post> {
    serde_json::from_str(&data).unwrap()
}

fn stringify(res: Result<Response<Body>, Error>) -> impl Future<Item = String, Error = Error> {
    res.unwrap().into_body().concat2().map(|chunk| {
        let v = chunk.to_vec();
        String::from_utf8_lossy(&v).to_string()
    })
}

fn main() {
    let url = "http://jsonplaceholder.typicode.com/posts"
        .parse::<hyper::Uri>()
        .unwrap();

    rt::run(fetch_url(url));
}
