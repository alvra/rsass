use nom::is_alphanumeric;
use spacelike::spacelike;
use std::fmt;
use std::str::from_utf8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector(Vec<SelectorPart>);

impl Selector {
    pub fn join(&self, other: &Selector) -> Selector {
        let mut split = other.0.splitn(2, |p| p == &SelectorPart::BackRef);
        let o1 = split.next().unwrap();
        if let Some(o2) = split.next() {
            let mut result = o1.to_vec();
            result.extend(self.0.iter().cloned());
            result.extend(o2.iter().cloned());
            Selector(result)
        } else {
            let mut result = self.0.clone();
            if !other.0.first().map(|p| p.is_operator()).unwrap_or(false) {
                result.push(SelectorPart::Descendant);
            }
            result.extend(other.0.iter().cloned());
            Selector(result)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectorPart {
    Simple(Vec<u8>),
    Descendant,
    RelOp(u8), // >, +, ~
    Attribute {
        name: Vec<u8>,
        op: Vec<u8>,
        val: Vec<u8>,
    },
    BackRef,
}

named!(pub selector<&[u8], Selector>,
       chain!(s: many1!(selector_part),
              || {
                  let mut s = s;
                  if s.last() == Some(&SelectorPart::Descendant) {
                      s.pop();
                  }
                  Selector(s)
              }));

named!(selector_part<&[u8], SelectorPart>,
       alt_complete!(
           chain!(r: take_while1!(is_selector_char),
                  || SelectorPart::Simple(r.to_vec())) |
           chain!(tag!("[") ~ spacelike? ~
                  name: take_while1!(is_selector_char) ~ spacelike? ~
                  op: alt_complete!(tag!("*=") | tag!("=")) ~ spacelike? ~
                  val: alt_complete!(
                      chain!(tag!("\"") ~ v: is_not!("\"") ~ tag!("\""),
                             || format!("\"{}\"", from_utf8(v).unwrap())) |
                      chain!(tag!("'") ~ v: is_not!("'") ~ tag!("'"),
                             || format!("'{}'", from_utf8(v).unwrap()))) ~
                  spacelike? ~
                  tag!("]"),
                  || SelectorPart::Attribute {
                      name: name.to_vec(),
                      op: op.to_vec(),
                      val: val.as_bytes().to_vec(),
                  }) |
           chain!(tag!("&"), || SelectorPart::BackRef) |
           chain!(spacelike? ~ tag!(">") ~ spacelike?,
                  || SelectorPart::RelOp(b'>')) |
           chain!(spacelike? ~ tag!("+") ~ spacelike?,
                  || SelectorPart::RelOp(b'+')) |
           chain!(spacelike? ~ tag!("~") ~ spacelike?,
                  || SelectorPart::RelOp(b'~')) |
           chain!(spacelike, || SelectorPart::Descendant)
           ));

fn is_selector_char(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'.' || chr == b'#'
}

impl SelectorPart {
    fn is_operator(&self) -> bool {
        match self {
            &SelectorPart::Simple(_) => false,
            &SelectorPart::Descendant => true,
            &SelectorPart::RelOp(_) => true,
            &SelectorPart::Attribute { .. } => false,
            &SelectorPart::BackRef => false,
        }
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        for ref p in &self.0 {
            try!(write!(out, "{}", p));
        }
        Ok(())
    }
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SelectorPart::Simple(ref s) => {
                write!(out, "{}", from_utf8(s).unwrap())
            }
            &SelectorPart::Descendant => write!(out, " "),
            &SelectorPart::RelOp(ref c) => {
                write!(out, " {} ", c.clone() as char)
            }
            &SelectorPart::Attribute { ref name, ref op, ref val } => {
                write!(out, "[{}{}{}]",
                       from_utf8(name).unwrap(),
                       from_utf8(op).unwrap(),
                       from_utf8(val).unwrap())
            }
            &SelectorPart::BackRef => write!(out, "&"),
        }
    }
}

#[cfg(test)]
mod test {
    use nom::IResult::*;
    use selectors::*;

    #[test]
    fn simple_selector() {
        assert_eq!(selector(b"foo "),
                   Done(&b""[..], Selector(
                       vec![SelectorPart::Simple(b"foo"[..].into())])))
    }

    #[test]
    fn selector2() {
        assert_eq!(selector(b"foo bar "),
                   Done(&b""[..], Selector(
                       vec![SelectorPart::Simple(b"foo"[..].into()),
                            SelectorPart::Descendant,
                            SelectorPart::Simple(b"bar"[..].into())])))
    }

    #[test]
    fn child_selector() {
        assert_eq!(selector(b"foo > bar "),
                   Done(&b""[..], Selector(
                       vec![SelectorPart::Simple(b"foo"[..].into()),
                            SelectorPart::RelOp(b'>'),
                            SelectorPart::Simple(b"bar"[..].into())])))
    }
}