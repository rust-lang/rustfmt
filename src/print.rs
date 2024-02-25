use std::io::Write;
use std::sync::{Arc, Mutex};
use rustc_errors::{ColorSpec, WriteColor, Color as RustColor};
use crate::Color;

#[derive(Clone)]
pub struct Printer {
    inner: Arc<Mutex<PrinterInner>>,
}

impl Write for Printer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut inner = self.inner.lock().unwrap();
        let col = inner.current_color;
        inner.messages.push(PrintMessage::Term(TermMessage::new(buf.to_vec(), col)));
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl WriteColor for Printer {
    #[inline]
    fn supports_color(&self) -> bool {
        self.inner.lock().unwrap().supports_color
    }

    #[inline]
    fn set_color(&mut self, spec: &ColorSpec) -> std::io::Result<()> {
        self.inner.lock().unwrap().current_color = spec.fg().copied();
        Ok(())
    }

    #[inline]
    fn reset(&mut self) -> std::io::Result<()> {
        self.inner.lock().unwrap().current_color.take();
        Ok(())
    }
}

struct PrinterInner {
    color_setting: Color,
    current_color: Option<RustColor>,
    messages: Vec<PrintMessage>,
    supports_color: bool,
}

impl Printer {

    pub fn new(term_output_color: Color) -> Self {
        Self {
            inner: Arc::new(Mutex::new(PrinterInner {
                color_setting: term_output_color,
                current_color: None,
                messages: vec![],
                supports_color: true, // Todo: Actually check
            })),
        }
    }

    #[inline]
    pub fn push_msg(&self, msg: PrintMessage) {
        self.inner.lock().unwrap().messages.push(msg);
    }
}

#[macro_export]
macro_rules! buf_println {
    ($pb: expr, $($arg:tt)*) => {{
        let mut msg_buf = Vec::new();
        let _ = writeln!(&mut msg_buf, $($arg)*);
        $pb.push_msg($crate::print::PrintMessage::Stdout(msg_buf));
    }};
}

#[macro_export]
macro_rules! buf_eprintln {
    ($pb: expr, $($arg:tt)*) => {{
        let mut msg_buf = Vec::new();
        let _ = writeln!(&mut msg_buf, $($arg)*);
        $pb.push_msg($crate::print::PrintMessage::StdErr(msg_buf));
    }};
}

#[macro_export]
macro_rules! buf_term_println {
    ($pb: expr, $col:expr, $($arg:tt)*) => {{
        let mut msg_buf = Vec::new();
        let _ = writeln!(&mut msg_buf, $($arg)*);
        $pb.push_msg($crate::print::PrintMessage::Term($crate::print::TermMessage::new(msg_buf, $col)));
    }};
}

pub enum PrintMessage {
    Stdout(Vec<u8>),
    StdErr(Vec<u8>),
    Term(TermMessage)
}

pub struct TermMessage {
    message: Vec<u8>,
    color: Option<RustColor>,
}

impl TermMessage {
    pub fn new(message: Vec<u8>, color: Option<RustColor>) -> Self {
        Self { message, color }
    }
}
