use crate::grapheme::{Grapheme, Graphemes};

#[derive(Clone, Debug, PartialEq)]
pub struct TextBuffer {
    pub buf: Graphemes,
    pub position: usize,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            // Set cursor
            buf: Graphemes::new(" "),
            position: 0,
        }
    }

    pub fn text(&self) -> Graphemes {
        let mut ret = self.buf.clone();
        ret.pop();
        ret
    }

    fn is_head(&self) -> bool {
        self.position == 0
    }

    fn is_tail(&self) -> bool {
        self.position == self.buf.len() - 1
    }

    fn replace(&mut self, new: &Graphemes) {
        self.buf = new.clone();
        self.buf.push(Grapheme::new(' '));
        self.to_tail();
    }

    pub fn insert(&mut self, ch: Grapheme) -> [Self; 2] {
        let prev = self.clone();
        self.buf.insert(self.position as usize, ch);
        self.next();
        [prev, self.clone()]
    }

    pub fn overwrite(&mut self, ch: Grapheme) -> [Self; 2] {
        let prev = self.clone();
        if self.is_tail() {
            self.insert(ch)
        } else {
            self.buf.splice(
                self.position as usize..(self.position + 1) as usize,
                vec![ch],
            );
            self.next();
            [prev, self.clone()]
        }
    }

    pub fn erase(&mut self) -> [Self; 2] {
        let prev = self.clone();
        if !self.is_head() {
            self.prev();
            self.buf
                .drain(self.position as usize..(self.position + 1) as usize);
        }
        [prev, self.clone()]
    }

    pub fn to_head(&mut self) -> [Self; 2] {
        let prev = self.clone();
        self.position = 0;
        [prev, self.clone()]
    }

    pub fn to_tail(&mut self) -> [Self; 2] {
        let prev = self.clone();
        self.position = self.buf.len() - 1;
        [prev, self.clone()]
    }

    pub fn prev(&mut self) -> [Self; 2] {
        let prev = self.clone();
        if !self.is_head() {
            self.position -= 1;
        }
        [prev, self.clone()]
    }

    pub fn next(&mut self) -> [Self; 2] {
        let prev = self.clone();
        if !self.is_tail() {
            self.position += 1;
        }
        [prev, self.clone()]
    }
}

#[cfg(test)]
mod test {
    mod erase {
        use super::super::*;

        #[test]
        fn test_for_empty() {
            let txt = TextBuffer::new();
            assert_eq!(Graphemes::new(" "), txt.buf);
            assert_eq!(0, txt.position);
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("bc "),
                position: 0, // indicate `b`.
            };
            let diff = txt.erase();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_tail() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("ab "),
                position: 2, // indicate tail.
            };
            let diff = txt.erase();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_head() {
            let txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            assert_eq!(Graphemes::new("abc "), txt.buf);
            assert_eq!(0, txt.position);
        }
    }

    mod insert {
        use super::super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextBuffer::new();
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("d "),
                position: 1, // indicate tail.
            };
            let diff = txt.insert(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("adbc "),
                position: 2, // indicate `b`.
            };
            let diff = txt.insert(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_tail() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abcd "),
                position: 4, // indicate tail.
            };
            let diff = txt.insert(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_head() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("dabc "),
                position: 1, // indicate `a`.
            };
            let diff = txt.insert(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }
    }

    mod overwrite {
        use super::super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextBuffer::new();
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("d "),
                position: 1, // indicate tail.
            };
            let diff = txt.overwrite(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("adc "),
                position: 2, // indicate `c`.
            };
            let diff = txt.overwrite(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_tail() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abcd "),
                position: 4, // indicate tail.
            };
            let diff = txt.overwrite(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_head() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("dbc "),
                position: 1, // indicate `b`.
            };
            let diff = txt.overwrite(Grapheme::new('d'));
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }
    }

    mod prev {
        use super::super::*;

        #[test]
        fn test_for_empty() {
            let txt = TextBuffer::new();
            assert_eq!(Graphemes::new(" "), txt.buf);
            assert_eq!(0, txt.position);
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            let diff = txt.prev();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_tail() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 2, // indicate `c`.
            };
            let diff = txt.prev();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_head() {
            let txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            assert_eq!(Graphemes::new("abc "), txt.buf);
            assert_eq!(0, txt.position);
        }
    }

    mod next {
        use super::super::*;

        #[test]
        fn test_for_empty() {
            let txt = TextBuffer::new();
            assert_eq!(Graphemes::new(" "), txt.buf);
            assert_eq!(0, txt.position);
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 2, // indicate `c`.
            };
            let diff = txt.next();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_tail() {
            let txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            assert_eq!(Graphemes::new("abc "), txt.buf);
            assert_eq!(3, txt.position);
        }

        #[test]
        fn test_at_head() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let diff = txt.next();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }
    }

    mod to_head {
        use super::super::*;

        #[test]
        fn test_for_empty() {
            let txt = TextBuffer::new();
            assert_eq!(Graphemes::new(" "), txt.buf);
            assert_eq!(0, txt.position);
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            let diff = txt.to_head();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_tail() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            let diff = txt.to_head();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_head() {
            let txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            assert_eq!(Graphemes::new("abc "), txt.buf);
            assert_eq!(0, txt.position);
        }
    }

    mod to_tail {
        use super::super::*;

        #[test]
        fn test_for_empty() {
            let txt = TextBuffer::new();
            assert_eq!(Graphemes::new(" "), txt.buf);
            assert_eq!(0, txt.position);
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 1, // indicate `b`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            let diff = txt.to_tail();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }

        #[test]
        fn test_at_tail() {
            let txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            assert_eq!(Graphemes::new("abc "), txt.buf);
            assert_eq!(3, txt.position);
        }

        #[test]
        fn test_at_head() {
            let mut txt = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 0, // indicate `a`.
            };
            let old = txt.clone();
            let new = TextBuffer {
                buf: Graphemes::new("abc "),
                position: 3, // indicate tail.
            };
            let diff = txt.to_tail();
            assert_eq!(new.buf, txt.buf);
            assert_eq!(new.position, txt.position);
            assert_eq!(diff, [old, new]);
        }
    }
}
