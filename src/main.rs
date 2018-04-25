#![warn(unused)]

#[macro_use]
extern crate log;
extern crate loggerv;
extern crate clap;
extern crate ssl_expiration;

use clap::{Arg, App};

mod endpoint;

use endpoint::Endpoint;
use ssl_expiration::SslExpiration;


fn main() {
    let matches = App::new("certificate-audit")
        .version("0.1.0")
        .author("Brent Montague <bigbam505@gmail.com>")
        .about("Validates ssl certificates")
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .arg(Arg::with_name("url")
             .short("u")
             .required(true)
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

    let endpoint = Endpoint::new(matches.value_of("url").unwrap());

    info!("Parsed url: {}", endpoint.address);

    let expiration = SslExpiration::from_domain_name(&endpoint.address).unwrap();
    let days = expiration.days();

    if expiration.is_expired() {
        println!("{}: expired", endpoint.address)
    } else {
        println!("{}: expires in {} days", endpoint.address, days)
    }
}
