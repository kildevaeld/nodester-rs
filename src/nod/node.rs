
use std::io::Write;
use std::path::Path;
use path::WorkDir;
use std::fs::File;
use std::fs;
use utils::{getarch, getplatform};
use url::Url;
use rustc_serialize::json;
use fetch;
use teewriter::TeeWriter;
use version::Version;
use platform::{Platform, Arch};
use error::Result;
use reqwest::header::ContentLength;

const NODE_URL: &'static str = "http://nodejs.org";


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
        println!("url {}", url);
        let headers = fetch::download_header(url)?;

        let len = headers
            .get::<ContentLength>()
            .map(|ct_len| **ct_len)
            .unwrap_or(0);


        Ok(len)

    }

    pub fn download<T: Write>(&self, version: &Version, write: T) -> Result<()> {

        let mut path = self.path.cache();
        path.push(version.get_cache_name(&self.platform, &self.arch, false));

        if path.exists() {
            return Ok(());
        }

        let file = File::create(path)?;

        let mut writer = TeeWriter::new(&file, write);

        let mut url = self.url.clone();
        url.set_path(&*version.get_url(&self.platform, &self.arch, false));

        fetch::download_to(url.as_str(), &file)?;

        writer.flush();

        Ok(())
    }

    pub fn available_versions(&self) -> Result<Vec<Version>> {
        let mut u = self.url.clone();
        u.set_path("dist/index.json");

        let result = fetch::download(u.as_str())?;

        let as_string = try!(String::from_utf8(result));
        let decoded: Vec<Version> = try!(json::decode(&as_string));
        Ok(decoded)

    }

    pub fn installed_versions(&self) -> Result<Vec<String>> {
        let mut dest = self.path.destination();
        let mut out = Vec::new();
        for entry in try!(dest.read_dir()) {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let s = path.file_name().unwrap().to_str().unwrap().to_string();
            out.push(s);
            //print!("{}\n", path.file_name().unwrap().to_str().unwrap());

        }

        out.sort_by(|a, b| b.cmp(a));
        Ok(out)
    }

    pub fn clean_cache(&self) -> Result<u64> {

        let cacheDir = self.path.cache();

        let files = try!(cacheDir.read_dir());

        let mut size: u64 = 0;

        for file in files {
            let entry = try!(file);
            let path = entry.path();

            let meta = entry.metadata()?;

            size += meta.len();

            if path.is_file() {
                try!(fs::remove_file(path));
            }

        }

        Ok(size)
    }
}