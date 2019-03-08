use platform::{Platform, Arch};

#[derive(RustcDecodable, Debug, Clone)]
pub struct Version {
    pub version: String,
    date: String,
    //files: Vec<String>
}

impl Version {
    pub fn get_url(&self, platform: &Platform, arch: &Arch, source: bool) -> String {
        format!(
            "dist/{}/{}",
            self.version,
            self.get_cache_name(platform, arch, source)
        )
    }


    pub fn get_cache_name(&self, platform: &Platform, arch: &Arch, source: bool) -> String {
        if source {
            format!("node-{}.tar.gz", self.version)
        } else {
            format!("node-{}-{}-{}.tar.gz", self.version, platform, arch)
        }

    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        self.version
    }
}