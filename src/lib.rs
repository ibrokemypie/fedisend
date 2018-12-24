extern crate blake2;
extern crate json;
extern crate reqwest;
extern crate url;

use blake2::{Blake2b, Digest};
use std::collections::HashMap;

pub struct Endpoints {
    pub misskey_key: String,
    pub mastodon_key: String,
    pub misskey_url: String,
    pub mastodon_url: String,
}

pub fn submit(api: &Endpoints, status: &String) {
    // mastodon api, pleroma etc
    {
        let fedi_parems = [("visibility", "unlisted"), ("status", status)];
        let idempotency = Blake2b::new()
            .chain(status)
            .chain(&api.mastodon_key)
            .chain(&api.mastodon_url)
            .result();

        match reqwest::Client::new()
            .post(&api.mastodon_url.to_string())
            .header("idempotency-key", format!("{:x}", idempotency))
            .header("authorization", api.mastodon_key.to_owned())
            .form(&fedi_parems)
            .send()
        {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        };
    }

    // misskey api
    {
        let mut misskey_parems = HashMap::new();
        misskey_parems.insert("visibility", "home");
        misskey_parems.insert("text", &status);
        misskey_parems.insert("i", &api.misskey_key);

        let client = reqwest::Client::new()
            .post(&api.misskey_url.to_string())
            // TODO: replace with normal form when misskey supports it
            .json(&misskey_parems);

        match client.send() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        };
    }
}
