#[derive(Debug, PartialEq)]
pub struct Console<'a, W>
where
    W: std::io::Write,
{
    is_tty: bool,
    space: Option<crate::color::Space>,
    writer: &'a W,
}

impl<'a, W> Console<'a, W>
where
    W: std::io::Write + std::os::unix::io::AsRawFd,
{
    pub fn from_os(writer: &W) -> Console<W> {
        Console {
            is_tty: is_tty(writer),
            space: detect_space(),
            writer,
        }
    }
}

impl<'a, W> Console<'a, W>
where
    W: std::io::Write,
{
    pub fn from_writer(writer: &W, is_tty: bool, space: Option<crate::color::Space>) -> Console<W> {
        Console {
            is_tty,
            space,
            writer,
        }
    }
}

fn detect_space() -> Option<crate::color::Space> {
    if let Ok(term) = std::env::var("COLORTERM") {
        let term = term.trim().to_lowercase();
        if &term == "24bit" || &term == "truecolor" {
            return Some(crate::color::Space::Bits24);
        }
    }

    if let Ok(term) = std::env::var("TERM") {
        if let Some((_, term)) = term.trim().to_lowercase().rsplit_once("-") {
            if term == "dumb" || term == "unknown" {
                return None;
            } else if term == "16color" {
                return Some(crate::color::Space::Bits4);
            } else if term == "256color" {
                return Some(crate::color::Space::Bits8);
            }
        }
    }

    return None;
}

fn is_tty<W>(writer: &W) -> bool
where
    W: std::io::Write + std::os::unix::io::AsRawFd,
{
    let fd = writer.as_raw_fd();
    unsafe { libc::isatty(fd) != 0 }
}

#[cfg(test)]
mod test_console {
    use super::Console;

    #[test]
    fn from_os() {
        let writer = std::io::stdout();
        Console::from_os(&writer);
    }

    #[test]
    fn from_writer() {
        let writer = std::io::Cursor::new(Vec::new());
        let console = Console::from_writer(&writer, false, None);
        let expected = Console {
            is_tty: false,
            space: None,
            writer: &writer,
        };
        assert_eq!(console, expected);
    }
}
