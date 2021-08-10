use anyhow::Result;
use std::{convert::TryInto, fs::File, io::Read, path::Path};

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;

    let mut content = String::with_capacity(file.metadata()?.len().try_into()?);

    file.read_to_string(&mut content)?;

    Ok(content)
}
