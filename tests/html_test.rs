use anyhow::Result;
use insta::assert_snapshot;
use std::{fs::File, io::Read};

use hot_curry::resolve_html_assets;

fn open_html_input() -> Result<Vec<u8>> {
    let mut file = File::open("./tests/resources/index.html")?;

    let mut html = Vec::new();

    file.read_to_end(&mut html)?;

    Ok(html)
}

#[test]
fn it_prepends_base_url_to_relative_paths() -> Result<()> {
    let base_url = "https://fake.com/static/";

    let html = open_html_input()?;

    let new_html = resolve_html_assets(base_url, &mut html.as_slice())?;

    assert_snapshot!(new_html);

    Ok(())
}
