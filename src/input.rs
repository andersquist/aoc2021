use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::Path,
    str::FromStr,
};

pub fn parse<'a, T>(path: &'a Path) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_reader(
        reader,
        path.file_name()
            .expect("couldn't open file")
            .to_string_lossy(),
    )
}

pub fn parse_str<'a, T>(data: &'a str) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    parse_reader(Cursor::new(data), "dummy file")
}

pub fn parse_reader<'a, T, Reader, Filename>(
    mut reader: Reader,
    file_name: Filename,
) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
    Reader: 'a + BufRead,
    Filename: 'a + std::fmt::Display,
{
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
                        eprintln!("{}:{}: {} for {:?}", file_name, line, e, buf);
                        None
                    }
                }
            }
        })
    })
    .fuse())
}

pub fn parse_newline<'a, T>(path: &'a Path) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_newline_reader(
        reader,
        path.file_name()
            .expect("couldn't open file")
            .to_string_lossy(),
    )
}

pub fn parse_newline_str<'a, T>(data: &'a str) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    parse_newline_reader(Cursor::new(data), "dummy file")
}

pub fn parse_newline_reader<'a, T, Reader, Filename>(
    mut reader: Reader,
    file_name: Filename,
) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
    Reader: 'a + BufRead,
    Filename: 'a + std::fmt::Display,
{
    let mut buf = String::new();
    let mut line = 0;

    fn detect_newline(buf: &str) -> bool {
        let new_line = ["\n\n", "\n\r\n"];
        new_line.iter().any(|n| {
            buf.as_bytes()
                .iter()
                .rev()
                .zip(n.as_bytes().iter())
                .all(|(a, b)| a == b)
        })
    }

    Ok(std::iter::from_fn(move || {
        buf.clear();
        while buf.is_empty() || !detect_newline(&buf) {
            line += 1;
            if reader.read_line(&mut buf).ok()? == 0 {
                break;
            }
        }

        if buf.is_empty() {
            None
        } else {
            match T::from_str(&buf) {
                Ok(t) => Some(t),
                Err(e) => {
                    eprintln!("{}:{}: {} for {:?}", file_name, line - 1, e, buf,);
                    None
                }
            }
        }
    })
    .fuse())
}

pub struct Separated<T>(Vec<T>);

impl<T> FromStr for Separated<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(Separated)
    }
}

impl<T> IntoIterator for Separated<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
