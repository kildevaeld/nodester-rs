
use colored::*;
use nod::{NodError, Node, Progress, Result};
use pbr::{ProgressBar, Units};

struct CliProgress<'a> {
    pb: &'a mut ProgressBar<std::io::Stdout>,
}

impl<'a> CliProgress<'a> {
    pub fn new(pb: &'a mut ProgressBar<std::io::Stdout>) -> CliProgress<'a> {
        CliProgress { pb }
    }
}

impl<'a> Progress for CliProgress<'a> {
    fn progress(&mut self, progress: u64, _total: u64) {
        self.pb.add(progress);
    }
}


pub fn use_cmd(node: &Node, version: &str) -> Result<()> {

    let version = node.get_version(version);

    if version.is_none() {
        return Err(NodError::Other("no version"));
    }

    let v = version.unwrap();

    let size = node.download_size(&v)?;

    let mut pb = ProgressBar::new(size);

    pb.set_units(Units::Bytes);
    pb.set_width(Some(80));
    println!("Downloading: {}", v.version.bold());
    {
        let mut progress = CliProgress::new(&mut pb);
        node.download(&v, &mut progress)?;
    }

    let done = format!("Download  ... {}", "done".green());
    pb.finish_print(&done);

    print!("\nUnpacking ... ");
    node.unpack(&v)?;
    print!("{}", "done".green());
    node.link(&v)?;

    Ok(())
}


pub fn run_cmd(node: &Node) -> Result<()> {

    Ok(())
}

pub fn list_cmd(node: &Node) -> Result<()> {
    let locals = node.installed_versions()?;

    if locals.len() == 0 {
        println!("No installed version yet.");
        return Ok(());
    }

    println!("{}", "Installed version:".bold());
    for v in locals {
        println!("{}", v);
    }
    Ok(())
}

pub fn listremote_cmd(node: &Node, count: usize) -> Result<()> {
    let locals = node.installed_versions()?;
    let versions = node.available_versions()?;
    let vs: Vec<String> = versions
        .iter()
        .take(count)
        .map(|x| x.version.clone())
        .collect();
    print!("{}:\n", "Node versions".bold());
    for v in vs {
        match locals.iter().position(|m| *m == v) {
            Some(_) => print!("{}\n", v.bold()),
            None => print!("{}\n", v),
        }

    }


    Ok(())
}

pub fn clear_cache(node: &Node) -> Result<()> {
    print!("Clearing cache ... ");
    let size = node.clean_cache()?;
    print!("done\nFreed {}kb\n", size);
    Ok(())
}