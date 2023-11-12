use ipnet::IpNet;
use ipnet::Ipv4Net;
use ipnet::Ipv4Subnets;
use iprange::IpRange;
use prefix_trie::*;
use rocket::fairing::AdHoc;
use rocket::response::content;
use rocket::response::content::RawJson;
use rocket::serde::json::{json, Json, Value};
use rocket::Request;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader, Error, Write};
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;
use String;
#[macro_use]
extern crate rocket;

#[derive(Debug, Default, Serialize, Deserialize)]
struct IPinfo {
    // #[serde(rename(serialize = "Ip", deserialize = "Ip"))]
    ip: String,
    // #[serde(rename(serialize = "Country", deserialize = "Country"))]
    country: String,
    // #[serde(rename(serialize = "Area", deserialize = "Area"))]
    area: String,
    // #[serde(rename(serialize = "City", deserialize = "City"))]
    city: String,
    // #[serde(rename(serialize = "Isp", deserialize = "Isp"))]
    isp: String,
}

type IPinfoList = Vec<IPinfo>;
#[derive(Debug, Default, Serialize, Deserialize)]
struct IPinfoResult {
    data: Vec<IPinfo>,
    count: u32,
    code: i32,
    msg: String,
}
type Messages<'r> = &'r State<IPinfoList>;

pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn read_all_file() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("content {}", content);
}

fn main111() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);

    let s = String::from("hello");

    let mut greeting = String::from("hello");
    let tmps = " beauty".to_string();

    greeting.push_str(&tmps);
    greeting.push('!');

    println!("tmps {}", tmps);
    println!("{}", greeting);

    let path = "data.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut pm = PrefixMap::<Ipv4Net, String>::new();
    //let prefix: Ipv4Net = "1.1.1.0/24".parse().unwrap();
    //let value: u32 = 100;
    //pm.insert(prefix, value);
    //let prefix1: Ipv4Net = "1.1.2.0/24".parse().unwrap();
    //let value1: u32 = 200;
    //pm.insert(prefix1, value1);
    //let res = pm.get(&prefix);
    //println!("{}", res.unwrap());
    let mut ipinfo = IPinfo::default();
    for line in buffered.lines().map(|x| x.unwrap()) {
        let lineinfo: Vec<&str> = line.split(" ").collect();
        if lineinfo.len() != 2 {
            println!("this line format error {}", line);
            continue;
        }
        let values: Vec<&str> = lineinfo[1].split("|").collect();
        println!("clause: {}", line);
        if values.len() == 4 {
            ipinfo.country = String::from(values[0]);
            ipinfo.area = String::from(values[1]);
            ipinfo.city = String::from(values[2]);
            ipinfo.isp = String::from(values[3]);
        }
        let ip_prefix = lineinfo[0].parse().unwrap();
        pm.insert(ip_prefix, lineinfo[1].to_string());
        println!("ipinfo: {:?}", ipinfo);
    }
    let input_args: Vec<String> = std::env::args().collect();
    if input_args.len() == 2 {
        let input_ip = &input_args[1];
        let net: Ipv4Net = input_ip.parse().unwrap();
        println!("net {} broadcast {}", net.network(), net.broadcast());
    }
    let ip_range: IpRange<Ipv4Net> = ["172.16.0.0/16", "192.168.1.0/24"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    for network in &ip_range {
        println!("{:?}", network);
    }
    let prefix_1 = Ipv4Net::new(Ipv4Addr::new(1, 0, 33, 0), 32);
    let res = pm.get_lpm(&prefix_1.unwrap());
    match res {
        Some(value) => {
            println!("{:?}", value);
            let (key, value) = value;
            println!("1.0.32.0 data: {:?} {}", key, value);
        }
        None => println!("none"),
    }
    if res.is_some() {
        let (key, value) = res.unwrap();
        println!("1.0.32.0 data: {:?} {}", key, value);
    } else {
        println!("1.2.33.0 not match");
    }
}

fn main222() {
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);

    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str("www.example.com.").unwrap();

    // NOTE: see 'Setup a connection' example above
    // Send the query and get a message response, see RecordType for all supported options
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();

    // Messages are the packets sent between client and server in DNS, DnsResonse's can be
    //  dereferenced to a Message. There are many fields to a Message, It's beyond the scope
    //  of these examples to explain them. See trust_dns::op::message::Message for more details.
    //  generally we will be interested in the Message::answers
    let answers: &[Record] = response.answers();

    // Records are generic objects which can contain any data.
    //  In order to access it we need to first check what type of record it is
    //  In this case we are interested in A, IPv4 address
    if let Some(RData::A(ref ip)) = answers[0].data() {
        println!("{}", *ip);
        assert_eq!(*ip, Ipv4Addr::new(93, 184, 216, 34))
    } else {
        assert!(false, "unexpected result")
    }
}

fn init_iptree() -> PrefixMap<Ipv4Net, String> {
    //let path = "/Users/guirongliu/ks/iplib/ipdetail/ipdetail";
    let path = "data.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut cnt: u32 = 0;
    let mut pm = PrefixMap::<Ipv4Net, String>::new();
    for line in buffered.lines().map(|x| x.unwrap()) {
        let lineinfo: Vec<&str> = line.split(" ").collect();
        if lineinfo.len() != 2 {
            println!("this line format error {}", line);
            continue;
        }
        let ip_prefix = lineinfo[0].parse().unwrap();
        pm.insert(ip_prefix, lineinfo[1].to_string());
        cnt += 1;
    }
    println!("ipdetail line count {}", cnt);

    return pm;
}

#[derive(FromForm)]
struct Options<'r> {
    ip: Option<&'r str>,
}
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[catch(404)]
fn not_found(request: &Request<'_>) -> content::RawHtml<String> {
    let html = match request.format() {
        Some(ref mt) if !(mt.is_xml() || mt.is_html()) => {
            format!("<p>'{}' requests are not supported.</p>", mt)
        }
        _ => format!(
            "<p>Sorry, '{}' is an invalid path! Try \
                  /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
            request.uri()
        ),
    };

    content::RawHtml(html)
}

#[get("/content")]
fn json() -> content::RawJson<&'static str> {
    content::RawJson(r#"{ "payload": "I'm here" }"#)
}
// Try visiting:
//   http://127.0.0.1:8000/ipinfo?ip=1.0.33.0,1.100.32.0
#[get("/?<opt..>")]
fn get_ipinfo(opt: Options<'_>, pm: &State<PrefixMap<Ipv4Net, String>>) -> String {
    //let mut resultlist = IPinfoList::default();
    let mut ip_result = IPinfoResult {
        data: IPinfoList::default(),
        count: 0,
        code: 0,
        msg: String::from("ok"),
    };
    if let Some(ip) = opt.ip {
        let ips: Vec<&str> = ip.split(",").collect();
        for _ip in ips {
            if let Ok(ip_value) = _ip.parse::<Ipv4Addr>() {
                println!("ip is {} ", ip_value);
                let prefix_1 = Ipv4Net::new(ip_value, 32);
                let res = pm.get_lpm(&prefix_1.unwrap());
                let mut ipinfo = IPinfo::default();
                match res {
                    Some(value) => {
                        let (key, val) = value;
                        let values: Vec<&str> = val.split("|").collect();
                        if values.len() == 4 {
                            ipinfo.ip = String::from(_ip);
                            ipinfo.country = String::from(values[0]);
                            ipinfo.area = String::from(values[1]);
                            ipinfo.city = String::from(values[2]);
                            ipinfo.isp = String::from(values[3]);
                            let s: String = serde_json::to_string(&ipinfo).unwrap();
                        }
                        //resultlist.push(ipinfo);
                        ip_result.data.push(ipinfo);
                        ip_result.count += 1;
                    }
                    None => {
                        println!("none");
                    }
                }
            } else {
                ip_result.data = IPinfoList::default();
                ip_result.count = 0;
                ip_result.code = -1;
                ip_result.msg = format!("{} format error", _ip);
                println!("ip {} format error ", _ip);
                return serde_json::to_string_pretty(&ip_result).unwrap();
            }
        }
    }

    //json!({"data": resultlist })
    //serde_json::to_string_pretty({"data": resultlist });
    serde_json::to_string_pretty(&ip_result).unwrap()
}

#[launch]
fn rocket() -> _ {
    println!("start web....");
    let g_pm = init_iptree();
    rocket::build()
        .mount("/ipinfo", routes![get_ipinfo])
        .mount("/wave", routes![wave])
        .mount("/content", routes![json])
        .register("/", catchers![not_found])
        .manage(g_pm)
}
