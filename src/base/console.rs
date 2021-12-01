pub struct Console<'a, W>
where
    W: std::io::Write,
{
    is_tty: bool,
    storage: Option<crate::base::color::storage::Storage>,
    writer: &'a mut W,
}

impl<'a, W> Console<'a, W>
where
    W: std::io::Write,
{
    pub fn columns(&self) -> u8 {
        if let Ok(columns) = std::env::var("COLUMNS") {
            if let Ok(columns) = u8::from_str_radix(&columns, 10) {
                return columns;
            }
        }

        80
    }

    pub fn lines(&self) -> u8 {
        if let Ok(lines) = std::env::var("LINES") {
            if let Ok(lines) = u8::from_str_radix(&lines, 10) {
                return lines;
            }
        }

        25
    }

    pub fn render<R>(&mut self, component: &R) -> std::io::Result<()>
    where
        R: crate::base::render::Render,
    {
        let options = crate::base::render::Options {
            columns: self.columns(),
            lines: self.lines(),
        };

        for (text, style) in component.render(options).iter() {
            let text = match self.storage {
                Some(storage) => style.render(text, storage),
                None => text.to_string(),
            };

            self.writer.write(text.as_bytes())?;
        }

        Ok(())
    }
}

impl<'a, W> Console<'a, W>
where
    W: std::io::Write + std::os::unix::io::AsRawFd,
{
    pub fn from_os(writer: &mut W) -> Console<W> {
        Console {
            is_tty: is_tty(writer),
            storage: detect_storage(),
            writer,
        }
    }
}

impl<'a, W> Console<'a, W>
where
    W: std::io::Write,
{
    pub fn from_writer(
        writer: &mut W,
        is_tty: bool,
        storage: Option<crate::base::color::storage::Storage>,
    ) -> Console<W> {
        Console {
            is_tty,
            storage,
            writer,
        }
    }
}

fn detect_storage() -> Option<crate::base::color::storage::Storage> {
    if let Ok(term) = std::env::var("COLORTERM") {
        let term = term.trim().to_lowercase();
        if &term == "24bit" || &term == "truecolor" {
            return Some(crate::base::color::storage::Storage::Bits24);
        }
    }

    if let Ok(term) = std::env::var("TERM") {
        if let Some((_, term)) = term.trim().to_lowercase().rsplit_once("-") {
            if term == "dumb" || term == "unknown" {
                return None;
            } else if term == "16color" {
                return Some(crate::base::color::storage::Storage::Bits4);
            } else if term == "256color" {
                return Some(crate::base::color::storage::Storage::Bits8);
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
        let mut writer = std::io::stdout();
        Console::from_os(&mut writer);
    }

    #[test]
    fn from_writer() {
        let mut writer = std::io::Cursor::new(Vec::new());
        let console = Console::from_writer(&mut writer, false, None);
        let mut writer = std::io::Cursor::new(Vec::new());
        let expected = Console {
            is_tty: false,
            storage: None,
            writer: &mut writer,
        };
        assert_eq!(console.is_tty, expected.is_tty);
        assert_eq!(console.storage, expected.storage);
    }
}
