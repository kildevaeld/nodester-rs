use super::platform::{Arch, Platform};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn get_dist_name(&self, platform: &Platform, arch: &Arch, source: bool) -> String {
        if source {
            format!("node-{}", self.version)
        } else {
            format!("node-{}-{}-{}", self.version, platform, arch)
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        self.version
    }
}