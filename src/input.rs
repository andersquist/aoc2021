use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub fn parse<T>(path: &Path) -> std::io::Result<impl '_ + Iterator<Item = T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut line = 0;
    Ok(std::iter::from_fn(move || {
        buf.clear();
        reader.read_line(&mut buf).ok().and_then(|_| {
            line += 1;
            if buf.is_empty() {
                None
            } else {
                match T::from_str(&buf.trim()) {
                    Ok(t) => Some(t),
                    Err(e) => {
                        eprintln!(
                            "{}:{}: {} for {:?}",
                            path.file_name().expect("").to_string_lossy(),
                            line,
                            e,
                            buf
                        );
                        None
                    }
                }
            }
        })
    })
    .fuse())
}
