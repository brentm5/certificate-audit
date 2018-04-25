#![warn(unused)]

#[macro_use]
extern crate log;
extern crate loggerv;
extern crate clap;
extern crate ssl_expiration;
extern crate yaml_rust;

mod endpoint;

use clap::{Arg, App};
use endpoint::Endpoint;
use ssl_expiration::SslExpiration;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let matches = App::new("certificate-audit")
        .version("0.1.0")
        .author("Brent Montague <bigbam505@gmail.com>")
        .about("Validates ssl certificates")
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .arg(Arg::with_name("config")
             .short("c")
             .value_name("FILE")
             .help("Use an input file to load urls")
             .takes_value(true))
        .arg(Arg::with_name("url")
             .short("u")
             .value_name("URL")
             .help("Sets the url to check")
             .takes_value(true))
        .get_matches();

    loggerv::Logger::new()
        .verbosity(matches.occurrences_of("v"))
        .level(true)
        .line_numbers(true)
        .separator(" = ")
        .module_path(false)
        .colors(false)
        .init()
        .unwrap();

    if matches.is_present("url") {
        let endpoint = Endpoint::new(matches.value_of("url").unwrap());
        info!("Parsed url: {}", endpoint.address);

        let expiration = SslExpiration::from_domain_name(&endpoint.address).unwrap();
        let days = expiration.days();

        if expiration.is_expired() {
            println!("{}: expired", endpoint.address)
        } else {
            println!("{}: expires in {} days", endpoint.address, days)
        }
    } else {
        let filename = matches.value_of("config").unwrap();
        info!("Loading file: {}", filename);

        let mut f = File::open(filename).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let docs = YamlLoader::load_from_str(&contents).unwrap();

        let doc = &docs[0];

        let sites = &doc["sites"];

        for site in sites.as_vec().unwrap()  {
            let endpoint = Endpoint::new(site.as_str().unwrap());
            let expiration = SslExpiration::from_domain_name(&endpoint.address).unwrap();
            let days = expiration.days();
            println!("{}: expires in {} days", endpoint.address, days)
        }
    }
}
