use std::sync::Mutex;
use crate::Color;

pub struct Printer {
    inner: Mutex<PrinterInner>,
}

struct PrinterInner {
    color: Color,
    messages: Vec<PrintMessage>
}

impl Printer {

    pub fn new(term_output_color: Color) -> Self {
        Self {
            inner: Mutex::new(PrinterInner {
                color: term_output_color,
                messages: vec![],
            }),
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
        $pb.push_msg($crate::print::PrintMessage::Term(TermMessage::new(msg_buf, $col)));
    }};
}

pub enum PrintMessage {
    Stdout(Vec<u8>),
    StdErr(Vec<u8>),
    Term(TermMessage)
}

pub struct TermMessage {
    message: Vec<u8>,
    color: Option<Color>,
}

impl TermMessage {
    pub fn new(message: Vec<u8>, color: Option<Color>) -> Self {
        Self { message, color }
    }
}
