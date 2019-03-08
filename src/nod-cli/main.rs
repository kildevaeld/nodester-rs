#[macro_use]
extern crate clap;
extern crate nod;
extern crate colored;
extern crate pbr;

mod cmds;
mod progress;

use std::io::Write;
use std::env;
use std::path::PathBuf;
use clap::ArgMatches;
use nod::{Node, error};
use cmds::*;
use progress::Progress;

const DEFAULT_ROOT: &'static str = ".nodester2";
const ENV_ROOT_KEY: &'static str = "NODESTER_ROOT";

fn get_nodester_root() -> Option<PathBuf> {
    let mut homedir = env::home_dir();
    if let Some(h) = homedir {
        let mut b = h.clone();
        b.push(DEFAULT_ROOT);
        homedir = Some(b);
    }
    if let Ok(root) = env::var(ENV_ROOT_KEY) {
        homedir = Some(PathBuf::from(root));
    }

    homedir
}


fn wrapped_main(matches: ArgMatches) -> nod::Result<()> {

    let root = get_nodester_root();
    if root.is_none() {
        return Err(error::NodError::Other("invalid root"));
    }

    let node = Node::new(root.unwrap())?;

    if let Some(matches) = matches.subcommand_matches("use") {
        return use_cmd(&node, matches.value_of("files").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("listremote") {
        return listremote_cmd(&node, 10);
    } else if let Some(matches) = matches.subcommand_matches("list") {
        return list_cmd(&node);
    } else if matches.is_present("version") {
        return run_cmd(&node);
    } else if let Some(matches) = matches.subcommand_matches("clear") {
        return clear_cache(&node);
    }

    Ok(())
}

fn main() {

    let matches = clap_app!(node_app => 
        (version: "0.1")
        (author: "Rasmus Kildevæld")
        (about: "")
        (@arg version: "run version")
        (@subcommand listremote =>
            (aliases: &["lsr"])
            (@arg count: -c --count)
        )
        (@subcommand list =>
            (aliases: &["ls"])
        )
        (@subcommand use =>
            (aliases: &["u"])
            (@arg files: +required "")
        )
        (@subcommand clear =>)
    ).get_matches();

    if let Err(ref err) = wrapped_main(matches) {
        writeln!(&mut std::io::stderr(), "{}", err).unwrap();
    }


}