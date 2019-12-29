pub struct Writer {
    indent_size: usize,
    current_indent_level: usize,
    out: String,
}

impl Writer {
    #[inline]
    pub fn new(indent_size: usize) -> Self {
        Self {
            indent_size,
            current_indent_level: 0,
            out: String::new(),
        }
    }

    #[inline]
    pub fn indent(&mut self) -> &mut Self {
        self.current_indent_level += 1;
        self
    }

    #[inline]
    pub fn outdent(&mut self) -> &mut Self {
        if self.current_indent_level > 0 {
            self.current_indent_level -= 1;
        }

        self
    }

    #[inline]
    pub fn newline(&mut self) -> &mut Self {
        self.out.push('\n');
        self
    }

    #[inline]
    pub fn writeln(&mut self, s: impl AsRef<str>) -> &mut Self {
        self.write_indent().write(s).newline()
    }

    #[inline]
    pub fn indent_writeln(&mut self, s: impl AsRef<str>) -> &mut Self {
        self.indent().writeln(s)
    }

    #[inline]
    pub fn outdent_writeln(&mut self, s: impl AsRef<str>) -> &mut Self {
        self.outdent().writeln(s)
    }

    #[inline]
    pub fn end(self) -> String {
        self.out
    }

    #[inline]
    fn write(&mut self, s: impl AsRef<str>) -> &mut Self {
        self.out.push_str(s.as_ref());
        self
    }

    #[inline]
    fn write_indent(&mut self) -> &mut Self {
        self.write(" ".repeat(self.current_indent_level * self.indent_size))
    }
}
