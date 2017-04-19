extern crate digitalocean;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::Domains;
use digitalocean::request::Request;
use digitalocean::values::DomainRecord;
use digitalocean::action::{Get, List, Create, Update, Delete};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let domain = "example.com";
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}/records", domain);

    let req: Request<List, Vec<DomainRecord>> = Domains::get(domain)
        .records();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let domain = "example.com";
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}/records", domain);
    let (kind, name, data, ttl) = ("A", "www", "192.168.0.1", 100);

    let req: Request<Create, DomainRecord> = Domains::get(domain)
        .records()
        .create(kind, name, data)
        .ttl(ttl);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": kind,
        "name": name,
        "data": data,
        "ttl": ttl,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let domain = "example.com";
    let id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}/records/{}", domain, id);

    let req: Request<Get, DomainRecord> = Domains::get(domain)
        .records()
        .get(id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn update_produces_correct_request() {
    before();

    let domain = "example.com";
    let id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}/records/{}", domain, id);
    let (kind, name, ttl) = ("SRV", "ww2", 200);

    let req: Request<Update, DomainRecord> = Domains::get(domain)
        .records()
        .update(id)
        .kind(kind)
        .name(name)
        .ttl(ttl);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": kind,
        "name": name,
        "ttl": ttl,
    }));
}

#[test]
fn delete_produces_correct_request() {
    before();

    let domain = "example.com";
    let id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}/records/{}", domain, id);

    let req: Request<Delete, ()> = Domains::get(domain)
        .records()
        .delete(id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}