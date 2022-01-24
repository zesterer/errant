use super::*;
use std::{
    fs,
    path::PathBuf,
    process::Termination,
};

pub struct Backtrace<E> {
    err: E,
    loc: &'static Location<'static>,
    backtrace: Vec<&'static Location<'static>>,
}

impl<E: fmt::Debug> Error for Backtrace<E> {
    type Payload = E;

    fn create(err: Self::Payload, loc: &'static Location<'static>) -> Self {
        Self {
            err,
            loc,
            backtrace: Vec::new(),
        }
    }

    fn handle_try(&mut self, loc: &'static Location) {
        self.backtrace.push(loc);
    }
}

impl<E: fmt::Debug> fmt::Debug for Backtrace<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ariadne::{Report, ReportKind, Label, Fmt, Color, Span};

        #[derive(Copy, Clone)]
        struct MySpan(&'static str, usize, usize);

        impl Span for MySpan {
            type SourceId = &'static str;
            fn source(&self) -> &Self::SourceId { &self.0 }
            fn start(&self) -> usize { self.1 }
            fn end(&self) -> usize { self.2 }
        }

        fn lineref_to_span(line: usize, col: usize, src: &str) -> Range<usize> {
            let mut idx = 0;

            let mut lines = src.lines();
            for line in (&mut lines).take(line.saturating_sub(1)) {
                idx += line.chars().count() + 1;
            }
            idx += col.saturating_sub(1);

            idx..idx + 1
        }

        const DIR: &'static str = env!("CARGO_MANIFEST_DIR");

        let (spans, sources): (Vec<_>, Vec<_>) = std::iter::once(&self.loc)
            .chain(self.backtrace.iter())
            .map(|loc| {
                let mut path = PathBuf::from(DIR);
                path.push(loc.file());
                let text = fs::read_to_string(path).unwrap();
                let Range { start, end } = lineref_to_span(loc.line() as usize, loc.column() as usize, &text);
                let span = MySpan(loc.file(), start, end);
                let src = (loc.file(), text);
                (span, src)
            })
            .unzip();
        let sources = ariadne::sources(sources);

        let mut report = Report::<MySpan>::build(ReportKind::Error, self.loc.file(), 0)
            .with_message(format!("{:?}", self.err))
            .with_label(Label::new(spans[0]).with_message("Error encountered here".fg(Color::Red)));

        for (i, span) in spans.iter().enumerate().skip(1) {
            report = report
                .with_label(Label::new(*span).with_message(format!(
                    "({}) {}",
                    i.fg(Color::Red),
                    "Then propagated here".fg(Color::Yellow),
                )));
        }

        let mut buf = Vec::new();

        report
            .finish()
            .write(sources, &mut buf)
            .unwrap();

        write!(f, "{}", core::str::from_utf8(&buf).unwrap())
    }
}

impl<E: fmt::Debug> Termination for Backtrace<E> {
    fn report(self) -> i32 {
        eprint!("{:?}", self);
        1
    }
}
