use super::error::{NodError, Result};
use super::fetch;
use super::path::WorkDir;
use super::platform::{Arch, Platform};
use super::teewriter::ProgressWriter;
use super::utils::{getarch, getplatform};
use super::version::Version;

use libarchive::archive::{self, ReadFilter, ReadFormat};
use libarchive::reader::Builder;
use libarchive::writer;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_json;

use url::Url;

#[cfg(windows)]
fn symlink<S: AsRef<Path>, D: AsRef<Path>>(source: S, dest: D) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(source, dest)
}

#[cfg(unix)]
fn symlink<S: AsRef<Path>, D: AsRef<Path>>(source: S, dest: D) -> std::io::Result<()> {
    std::os::unix::fs::symlink(source, dest)
}


// use reqwest::header::CONTENT;

const NODE_URL: &'static str = "http://nodejs.org";


pub trait Progress {
    fn progress(&mut self, progress: u64, total: u64);
}


pub struct Node {
    platform: Platform,
    arch: Arch,
    path: WorkDir,
    url: Url,
}

impl Node {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Node> {

        let arch = getarch()?;
        let platform = getplatform()?;

        let mut root = WorkDir::new(path);

        let url = Url::parse(NODE_URL)?;

        root.ensure()?;

        let n = Node {
            arch: arch,
            platform: platform,
            path: root,
            url: url,
        };

        Ok(n)
    }

    pub fn get_version<T: Into<String>>(&self, version: T) -> Option<Version> {
        let mut v = version.into();
        if !v.starts_with("v") {
            v = format!("v{}", v);
        }

        let versions = self.available_versions();

        if versions.is_err() {
            println!("{}", versions.err().unwrap());
            return None;
        }

        if let Some(v) = versions.unwrap().iter().find(|ver| ver.version == v) {
            return Some(v.clone());
        }

        None

    }

    pub fn download_size(&self, version: &Version) -> Result<(u64)> {

        let mut url = self.url.clone();
        let path = version.get_url(&self.platform, &self.arch, false);
        url.set_path(&*path);

        let headers = fetch::download_header(url)?;

        let len = headers
            .get("Content-Length")
            .map(|ct_len| ct_len.to_str().unwrap().parse::<u64>().unwrap())
            .unwrap_or(0);


        Ok(len)

    }

    pub fn download<P: Progress>(&self, version: &Version, write: &mut P) -> Result<()> {

        let mut path = self.path.cache();
        path.push(version.get_cache_name(&self.platform, &self.arch, false));

        if path.exists() {
            return Ok(());
        }

        let size = self.download_size(version)?;

        let mut file = File::create(path)?;


        {
            let writer = ProgressWriter::new(&mut file, write, size as u64);
            let mut url = self.url.clone();
            url.set_path(&*version.get_url(&self.platform, &self.arch, false));

            fetch::download_to(url.as_str(), writer)?;
        }

        file.flush()?;

        Ok(())
    }

    pub fn unpack(&self, version: &Version) -> Result<()> {

        let mut path = self.path.cache();
        path.push(version.get_cache_name(&self.platform, &self.arch, false));

        if !path.exists() {
            return Err(NodError::Other("version not downloaded"));
        }


        let mut builder = Builder::new();

        builder.support_format(ReadFormat::All)?;
        builder.support_filter(ReadFilter::All)?;

        let mut reader = builder.open_file(path)?;

        let mut opts = archive::ExtractOptions::new();
        opts.add(archive::ExtractOption::Time);

        let writer = writer::Disk::new();

        let out_path = self.path.destination();


        writer.write(&mut reader, Some(out_path.to_string_lossy().as_ref()))?;


        Ok(())
    }

    pub fn link(&self, version: &Version) -> Result<()> {
        let mut path = self.path.destination();
        path.push(version.get_dist_name(&self.platform, &self.arch, false));

        let current = self.path.current();
        if current.exists() {
            std::fs::remove_file(&current)?;
        }

        symlink(path, current)?;


        Ok(())
    }

    pub fn available_versions(&self) -> Result<Vec<Version>> {
        let mut u = self.url.clone();
        u.set_path("dist/index.json");

        let result = fetch::download(u.as_str())?;

        //let as_string = String::from_utf8(result)?;
        let decoded: Vec<Version> = serde_json::from_slice(&result)?;
        Ok(decoded)

    }

    pub fn installed_versions(&self) -> Result<Vec<String>> {
        let mut dest = self.path.destination();
        let mut out = Vec::new();
        for entry in dest.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let s = path.file_name().unwrap().to_str().unwrap().to_string();
            let k = s
                .replace("node", "")
                .replace(self.platform.to_string().as_str(), "")
                .replace(self.arch.to_string().as_str(), "")
                .replace("-", "");
            out.push(k);

        }

        out.sort_by(|a, b| b.cmp(a));
        Ok(out)
    }

    pub fn clean_cache(&self) -> Result<u64> {

        let cacheDir = self.path.cache();

        let files = cacheDir.read_dir()?;

        let mut size: u64 = 0;

        for file in files {
            let entry = file?;
            let path = entry.path();

            let meta = entry.metadata()?;

            size += meta.len();

            if path.is_file() {
                fs::remove_file(path);
            }

        }

        Ok(size)
    }
}