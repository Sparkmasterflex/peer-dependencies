extern crate termion;
extern crate regex;

extern crate serde;
extern crate serde_json;

use serde_json::Value;

use std::fs::File;
use std::io::Read;
use std::process::Command;

use termion::color;
use regex::Regex;

fn main() {
  let mut file = match File::open("package.json") {
    Err(why) => panic!("couldn't read package.json: {:?}", why),
    Ok(file) => file,
  };
  let mut data = String::new();
  file.read_to_string(&mut data)
    .expect("cannot read file to string");

  let json: Value = serde_json::from_str(&data)
    .expect("serde_json what? from what?");
  let mut packages = vec![];

  match json["peerDependencies"].as_object() {
    Some(peer_dependencies) => {
      print_block("Installing Peer Dependencies");

      for obj in peer_dependencies {
        let re    = Regex::new(r####"""####)
          .expect("re Regex::new not working");
        let no_at = Regex::new(r"\^")
          .expect("no_at Regex::new not working");

        let v1 = obj.1.to_string();
        let v2 = re.replace_all(&v1, "");
        let version = no_at.replace_all(&v2, "@");

        packages.push(format!("{}{}", obj.0, version));
      }

      packages.push(String::from("--no-save"));
      println!("npm install {}", packages.iter().cloned().collect::<String>());
      let args = &packages[..];

      Command::new("npm")
        .arg("install")
        .args(args)
        .status()
        .expect("npm install failed you dumb bastard");
    },
    None => {
      print_block("No peerDependencies found");
    }
  };

  print_block("Installing Dependencies");

  Command::new("npm")
    .arg("install")
    .status()
    .expect("npm install failed you dumb bastard");

  print_block("Done :)");
}

fn print_block(msg: &str) {
  let eq_len  = 40;
  let msg_len = msg.len();

  let space_around = (eq_len - msg_len) / 2;
  let mut prepend = String::new();
  let mut append  = String::new();
  for i in 1..space_around {
    prepend += " ";
    append  += " ";
  }

  println!("{cyan}========================================{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
  println!("{cyan}|{message}|{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset),
    message = format!("{}{}{}", prepend, msg, append)
  );
  println!("{cyan}========================================{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
}

