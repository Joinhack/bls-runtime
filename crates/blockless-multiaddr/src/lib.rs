#[macro_use]
mod macros;
mod parse;

pub use parse::*;

#[derive(Debug)]
pub struct Indicator<'a> {
    val: &'a [u8],
    begin: usize,
    end: usize,
}

#[derive(Debug)]
pub enum Error {
    Partial,
    InvalidToken,
}

#[derive(Debug)]
pub struct MultiAddr<'a> {
    paths: Vec<Indicator<'a>>,
}

impl<'a> Indicator<'a> {
    pub fn new(bs: &'a [u8], begin: usize, end: usize) -> Self {
        Self {
            val: &bs[begin..end],
            begin,
            end,
        }
    }

    pub fn value(&self) ->  &[u8] {
        self.val
    }

    pub fn begin(&self) ->  usize {
        self.begin
    }

    pub fn end(&self) ->  usize {
        self.end
    }
}

impl<'a> MultiAddr<'a> {
    pub fn schema(&self) -> Result<&str, Error> {
        if self.paths.len() > 0 {
            std::str::from_utf8(self.paths[0].value())
                .map_err(|_| Error::InvalidToken)
        } else {
            Err(Error::Partial)
        }
    }
}