
use reqwest;
use reqwest::StatusCode;
use reqwest::header::Headers;
use std::io;
use std::io::Write;
use error::{NodError, Result};
use url::Url;


pub fn download(url: &str) -> Result<Vec<u8>> {

    let mut writer: Vec<u8> = vec![];
    download_to(url, &mut writer)?;

    Ok(writer)

}



pub fn download_to<T: Write>(url: &str, mut writer: T) -> Result<()> {

    let mut res = reqwest::get(url)?;

    if !res.status().is_success() {
        println!("{}", res.status());
        return Err(NodError::Other("status not"));
    }

    io::copy(&mut res, &mut writer)?;

    Ok(())
}

pub fn download_header(url: Url) -> Result<Headers> {

    let res = reqwest::get(url.as_str())?;

    if !res.status().is_success() {
        return Err(NodError::Other("status not"));
    }

    

    Ok(res.headers().clone())
}