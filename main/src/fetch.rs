use std::fs::File;
use std::ops::RangeInclusive;
use std::path::Path;
use std::sync::Arc;

use quicli::prelude::{debug, error};
use reqwest::blocking::Client;
use reqwest::Url;

pub const AOC_BASE_URL: &'static str = "https://adventofcode.com";
pub const AOC_YEAR: i32 = 2021;

pub fn input_url(day: u32) -> String {
  format!("{}/{}/day/{}/input", AOC_BASE_URL, AOC_YEAR, day)
}

pub fn fetch_inputs(token: Option<String>, days_range: RangeInclusive<u32>) {
  let jar = reqwest::cookie::Jar::default();

  if let Some(t) = token {
    let cookie = format!("session={}", &t.as_str());
    let url = Url::parse(AOC_BASE_URL).unwrap();
    jar.add_cookie_str(cookie.as_str(), &url)
  }

  let client = Client::builder()
    .cookie_provider(Arc::new(jar))
    .build().unwrap();

  for day in days_range {
    let filename = format!("./input{:02}.txt", day);
    let path = Path::new(&filename);
    let mut file = File::create(path).unwrap();
    let request = client.get(input_url(day)).build().unwrap();
    debug!("Downloading day {} input: {:?}", day, request);
    let mut response = client.execute(request).unwrap();
    if response.status().is_success() {
      response.copy_to(&mut file).unwrap();
    } else {
      error!("Could not fetch {}: {}", filename, response.status());
    }
  }
}
