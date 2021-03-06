pub struct Console<'a, W>
where
    W: std::io::Write,
{
    options: crate::core::render::Options,
    writer:  &'a mut W,
}

impl<'a, W> Console<'a, W>
where
    W: std::io::Write,
{
    pub fn render<R>(&mut self, component: &R) -> std::io::Result<()>
    where
        R: crate::core::render::Render,
    {
        for segment in component.render(&self.options).iter() {
            self.writer.write(segment.as_bytes())?;
            self.newline()?;
        }

        Ok(())
    }

    pub fn newline(&mut self) -> std::io::Result<()> {
        self.writer.write(&[10])?;
        Ok(())
    }
}

impl<'a, W> Console<'a, W>
where
    W: std::io::Write + std::os::unix::io::AsRawFd,
{
    pub fn from_fd(writer: &mut W) -> Console<W> {
        let is_tty = is_tty(writer);
        let tty_size = tty_size(writer);

        Console {
            options: crate::core::render::Options {
                is_tty,
                columns: Some(tty_size.0),
                rows: Some(tty_size.1),
                storage: detect_storage(),
            },
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
        options: crate::core::render::Options,
    ) -> Console<W> {
        Console { options, writer }
    }
}

fn detect_storage() -> Option<crate::core::color::storage::Storage> {
    if let Ok(term) = std::env::var("COLORTERM") {
        let term = term.trim().to_lowercase();
        if &term == "24bit" || &term == "truecolor" {
            return Some(crate::core::color::storage::Storage::Bits24);
        }
    }

    if let Ok(term) = std::env::var("TERM") {
        if let Some((_, term)) = term.trim().to_lowercase().rsplit_once("-") {
            if term == "dumb" || term == "unknown" {
                return None;
            } else if term == "16color" {
                return Some(crate::core::color::storage::Storage::Bits4);
            } else if term == "256color" {
                return Some(crate::core::color::storage::Storage::Bits8);
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

fn tty_size<W>(writer: &W) -> (usize, usize)
where
    W: std::io::Write + std::os::unix::io::AsRawFd,
{
    let fd = writer.as_raw_fd();
    let mut size = libc::winsize {
        ws_row:    0,
        ws_col:    0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ.into(), &mut size) } != -1
        && size.ws_col > 0
        && size.ws_row > 0
    {
        (size.ws_col.into(), size.ws_row.into())
    } else {
        (
            crate::core::render::DEFAULT_COLUMNS,
            crate::core::render::DEFAULT_ROWS,
        )
    }
}

#[cfg(test)]
mod test_console {
    use super::Console;

    #[test]
    fn from_fd() {
        let mut writer = std::io::stdout();
        Console::from_fd(&mut writer);
    }

    #[test]
    fn from_writer() {
        let options = crate::core::render::Options {
            columns: Some(crate::core::render::DEFAULT_COLUMNS),
            is_tty:  false,
            storage: None,
            rows:    Some(crate::core::render::DEFAULT_ROWS),
        };
        let mut writer = std::io::Cursor::new(Vec::new());
        let console = Console::from_writer(&mut writer, options);
        let mut writer = std::io::Cursor::new(Vec::new());
        let expected = Console { options, writer: &mut writer };
        assert_eq!(console.options, expected.options);
    }
}
