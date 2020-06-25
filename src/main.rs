// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use std::any::type_name;
use std::fs::{File};
use std::io::Read;
use std::path::{Path, PathBuf};
use clap::{App, Arg};
use serde::{Serialize, Deserialize};

use dirs;
use glob;
use shellexpand;

fn default_version() -> String {
    String::from("v1")
}

fn default_domains() -> Vec<String> {
    vec![]
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(default = "default_version")]
    version: String,
    #[serde(default = "default_domains")]
    domains: Vec<String>,
}

fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

fn load_domains(configfile: &Path) -> Option<Config> {
    println!("load_domains: configfile={:?}", configfile);
    match File::open(configfile) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let config: Config = serde_yaml::from_str(&content).unwrap();
            Some(config)
        }
        Err(_e) => None
    }
}

fn wrx(configfile: &Path, domain_pns: Vec<String>, list: bool, host: bool, dig: bool, ssl: bool, whois: bool) -> Result<(), &str> {
    println!("configfile='{:?}' domain_pns={:?}", configfile, domain_pns);
    println!("list={} host={} dig={} ssl={} whois={}", list, host, dig, ssl, whois);
    let config = load_domains(configfile).unwrap();
    println!("config={:?}", config);
    println!("config.domains={:?}", config.domains);
    Ok(())
}

fn main() {
    let matches = App::new("wrx")
        .about("web request for x (host|dig|ssl|whois)")
        .arg(
            Arg::with_name("config")
                .about("default='~/.config/wrx/wrx.yml'; config to use")
                .short('c')
		.long("config")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .about("list about")
                .short('l')
                .long("list")
        )
        .arg(
            Arg::with_name("host")
                .about("host about")
                .short('h')
                .long("host")
        )
        .arg(
            Arg::with_name("dig")
                .about("dig about")
                .short('d')
                .long("dig")
        )
        .arg(
            Arg::with_name("ssl")
                .about("ssl about")
                .short('s')
                .long("ssl")
        )
        .arg(
            Arg::with_name("whois")
                .about("whois about")
                .short('w')
                .long("whois")
        )
        .arg(
            Arg::with_name("domain_pns")
                .about("default=['*']; list of one or more domain patterns")
                .index(1)
                .multiple(true)
        )
        .get_matches();

    let configfile = expand_tilde(Path::new(matches.value_of("config").unwrap_or("~/.config/wrx/wrx.yml"))).unwrap();
    let domain_pns = match matches.values_of_lossy("domain_pns") {
	Some(domain_pns) => domain_pns,
        None => vec![String::from("*")],
    };
    let list = matches.is_present("list");
    let host = matches.is_present("host");
    let dig = matches.is_present("dig");
    let ssl = matches.is_present("ssl");
    let whois = matches.is_present("whois");
    let result = match wrx(&configfile, domain_pns, list, host, dig, ssl, whois) {
        Ok(result) => println!("cool"),
        Err(_e) => println!("bomb"),
    };
}
