use anyhow::Result;
use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeRef};
use std::{convert::TryInto, fs::File, io::Read, path::Path};
use url::Url;

/// Mutates an element attribute by prepending the provided base url to its value
fn prepend_base_url(base_url: &Url, element: &ElementData, attribute_name: &str) {
    let mut attributes = element.attributes.borrow_mut();

    let attribute_value = attributes.get(attribute_name).and_then(|attribute_value| {
        if attribute_value.parse::<Url>().is_ok() {
            None
        } else {
            Some(attribute_value.to_string())
        }
    });

    if let Some(attribute_value) = attribute_value {
        attributes.insert(
            attribute_name,
            base_url.join(attribute_value.as_str()).unwrap().to_string(),
        );
    }
}

/// Walks through all the html nodes and prepend the base url when needed
fn walk(base_url: &Url, node: NodeRef) {
    if let Some(element) = node.as_element() {
        match element.name.local.as_ref() {
            "script" | "img" => prepend_base_url(base_url, element, "src"),
            "link" => prepend_base_url(base_url, element, "href"),
            _ => (),
        }
    }

    for node in node.children().into_iter() {
        walk(base_url, node);
    }
}

#[allow(dead_code)]
pub fn resolve_html_assets<R>(base_url: &str, r: &mut R) -> Result<String>
where
    R: Read,
{
    let base_url = base_url.parse()?;

    let node = parse_html().from_utf8().read_from(r)?;

    walk(&base_url, node.clone());

    Ok(node.to_string())
}

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;

    let mut content = String::with_capacity(file.metadata()?.len().try_into()?);

    file.read_to_string(&mut content)?;

    Ok(content)
}
