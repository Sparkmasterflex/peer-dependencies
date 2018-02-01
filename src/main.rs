extern crate termion;
extern crate regex;

extern crate serde;
extern crate serde_json;

use serde_json::Value;

use std::fs::File;
use std::env;
use std::io::Read;
use std::process::Command;

use termion::color;
use regex::Regex;

fn main() {
  println!("{cyan}========================================{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
  println!("{cyan}|     Installing Peer Dependencies     |{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
  println!("{cyan}========================================{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );

  let current_directory = env::current_dir()
    .expect("current directory is wrong")
    .display().to_string();


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

  for obj in json["peerDependencies"].as_object()
    .expect("as_object is all fucked up")
  {
    let re    = Regex::new(r####"""####)
      .expect("re Regex::new not working");
    let no_at = Regex::new(r"\^")
      .expect("no_at Regex::new not working");

    let v1 = obj.1.to_string();
    let v2 = re.replace_all(&v1, "");
    let version = no_at.replace_all(&v2, "@");

    packages.push(format!("{}{}", obj.0, version));
  }
  println!("npm install {}", packages.iter().cloned().collect::<String>());
  let args = &packages[..];

  Command::new("cd")
    .arg(current_directory)
    .status()
    .expect("npm install failed you dumb bastard");
  // Command::new("pwd")
  //   .status()
  //   .expect("npm install failed you dumb bastard");

  Command::new("npm")
    .arg("install")
    .args(args)
    .status()
    .expect("npm install failed you dumb bastard");

  println!("{cyan}========================================{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
  println!("{cyan}|         Installing Dependencies      |{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
  println!("{cyan}========================================{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );

  Command::new("npm")
    .arg("install")
    .status()
    .expect("npm install failed you dumb bastard");

  println!("{cyan}=============  DONE :) ================{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
}
