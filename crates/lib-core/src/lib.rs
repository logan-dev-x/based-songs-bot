use std::path::Path;

fn read_wav<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let _file = std::fs::File::open(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_exists() {
        let res = read_wav("Cargo.toml");
        assert!(res.is_ok());
    }

    #[test]
    fn file_not_exists() {
        let res = read_wav("foo.wav");
        assert!(res.is_err());
    }
}
