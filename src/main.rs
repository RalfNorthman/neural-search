use anyhow::Result;
use bytes::Buf;
use bytes::Bytes;
// use pdf_extract::extract_text;
use reqwest::blocking;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_html_string(url: &str) -> Result<Bytes> {
    let response = blocking::get(url)?;
    let body = response.bytes()?;
    Ok(body)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let url_arg = &args[1];
    let url = url_arg.trim();
    dbg!(url);

    let file_name_arg = &args[2];
    let file_name = file_name_arg.trim();
    dbg!(file_name);

    let html = get_html_string(url)?;
    dbg!(&html);
    let html_reader = html.reader();

    //  let contents = extract_text(&file_name)?;
    let contents = html2text::from_read(html_reader, 80);
    dbg!(&contents);
    let mut file = File::create(&file_name)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
