#[macro_use] extern crate nickel;
extern crate hyper;
extern crate regex;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use hyper::Client;
use hyper::header::{Headers, Host, UserAgent, Referer, Connection};
use std::collections::HashMap;
use std::io::Read;
use regex::Regex;

fn request_for_f4m(url :&str) -> Vec<u8> {
  let client = Client::new();
  let mut headers = Headers::new();
  headers.set(Referer("http://moon.hdkinoteatr.com".to_owned()));
  headers.set(Connection::keep_alive());
  headers.set(Host{
    hostname: "185.38.12.39".to_owned(),
    port: None
  });
  headers.set(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.110 Safari/537.36".to_owned()));
  let mut request = client.get(url).headers(headers).send().unwrap();
  let mut buffer: Vec<u8>;
  buffer = Vec::new();
  request.read_to_end(&mut buffer).unwrap();
  buffer
}

fn request_for_m3u8(url :&str) -> Vec<u8> {
  let client = Client::new();
  let mut headers = Headers::new();
  headers.set(Referer("http://moon.hdkinoteatr.com".to_owned()));
  headers.set(Connection::keep_alive());
  headers.set(Host{
    hostname: "185.38.12.39".to_owned(),
    port: None
  });
  headers.set(UserAgent("Mozilla/5.0(iPad; U; CPU iPhone OS 3_2 like Mac OS X; en-us) AppleWebKit/531.21.10 (KHTML, like Gecko) Version/4.0.4 Mobile/7B314 Safari/531.21.10".to_owned()));
  let mut request = client.get(url).headers(headers).send().unwrap();
  let mut buffer: Vec<u8>;
  buffer = Vec::new();
  request.read_to_end(&mut buffer).unwrap();
  buffer
}

fn crossdomain_handler<'a> (_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  let mut data = HashMap::<&str, &str>::new();
  data.insert("host", "http://hd-serials.tv:6767");
  res.set(MediaType::Xml);
  res.render("app/views/crossdomain.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();
    let f4m_route_regex = Regex::new("/stream/f4m/(?P<stream>[a-zA-Z0-9/.,-]+)").unwrap();
    let m3u8_route_regex = Regex::new("/stream/m3u8/(?P<stream>[a-zA-Z0-9/.,-]+)").unwrap();

    server.get(f4m_route_regex, middleware! { |req|
      let stream_param = req.param("stream").unwrap();
      let mut stream_link: String = "http://".to_owned();
      stream_link.push_str(&stream_param);
      request_for_f4m(&stream_link)
    });

    server.get(m3u8_route_regex, middleware! { |req|
      let stream_param = req.param("stream").unwrap();
      let mut stream_link: String = "http://".to_owned();
      stream_link.push_str(&stream_param);
      request_for_m3u8(&stream_link)
    });

    server.get("/crossdomain.xml", crossdomain_handler);

    server.listen("0.0.0.0:6767");
}
