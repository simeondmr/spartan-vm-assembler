use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Token {
    Iadd32(u32),
    Pushb(u32),
    Jmp(u32),
    Hlt(u32),
    SectionBss(u32),
    SectionData(u32),
    SectionText(u32),
    RESB(u32),
    RESW(u32),
    RESD(u32),
    NumberU32(u32, u32),
    Literal(u32, String),
    Label(u32, String),
    StringTok(u32, String),
    SingleElem(u32, char),
    EOF(u32),
    INIT(u32)
}

impl Token {
    pub fn line(&self) -> u32 {
        match self {
            Token::Iadd32(line) => *line,
            Token::Pushb(line) => *line,
            Token::Hlt(line) => *line,
            Token::Jmp(line) => *line,
            Token::SectionBss(line) => *line,
            Token::SectionData(line) => *line,
            Token::SectionText(line) => *line,
            Token::RESB(line) => *line,
            Token::RESW(line) => *line,
            Token::RESD(line) => *line,
            Token::NumberU32(line, _) => *line,
            Token::Literal(line, _) => *line,
            Token::Label(line, _) =>*line,
            Token::StringTok(line, _) => *line,
            Token::SingleElem(line, _) => *line,
            Token::EOF(line) => *line,
            Token::INIT(line) => *line,
        }
    }
}



impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Iadd32(_), Token::Iadd32(_)) => true,
            (Token::Pushb(_), Token::Pushb(_)) => true,
            (Token::Jmp(_), Token::Jmp(_)) => true,
            (Token::NumberU32(_, _), Token::NumberU32(_, _)) => true,
            (Token::Literal(_, _), Token::Literal(_, _)) => true,
            (Token::Label(_, _), Token::Label(_, _)) => true,
            (Token::StringTok(_, _), Token::StringTok(_, _)) => true,
            (Token::SingleElem(_, element0), Token::SingleElem(_, element1)) => if element0 == element1{ true } else { false },
            (Token::SectionBss(_), Token::SectionBss(_)) => true,
            (Token::SectionData(_), Token::SectionData(_)) => true,
            (Token::SectionText(_), Token::SectionText(_)) => true,
            (Token::RESB(_), Token::RESB(_)) => true,
            (Token::RESW(_), Token::RESW(_)) => true,
            (Token::EOF(_), Token::EOF(_)) => true,
            _ => false
        }
    }
}

pub struct Lexer {
    file: File,
    current_line: u32,
    keywords: HashMap<String, Token>
}

impl Lexer {
    pub fn new(file_name: String) -> Option<Lexer> {
        Some(Lexer {
            file: File::open(file_name).ok()?,
            current_line: 1,
            keywords: HashMap::from([
                ("iadd32".to_string(), Token::Iadd32(0)),
                ("pushb".to_string(), Token::Pushb(0)),
                ("hlt".to_string(), Token::Hlt(0)),
                ("jmp".to_string(), Token::Jmp(0)),
                ("section_bss".to_string(), Token::SectionBss(0)),
                ("section_data".to_string(), Token::SectionData(0)),
                ("section_text".to_string(), Token::SectionText(0)),
                ("resb".to_string(), Token::RESB(0)),
                ("resw".to_string(), Token::RESW(0))
            ])
        })
    }

    pub fn next_token(&mut self) -> Token {
        let mut buf = [0];

        while self.file.read_exact(&mut buf).is_ok() {
            let c = buf[0] as char;

            if c == '\n' {
                self.current_line += 1;
            }

            if c.is_whitespace() {
                continue;
            }

            //println!("Value: {}", c);

            if c.is_ascii_digit() {
                let mut number = c.to_string();
                let mut last_char = None;

                while self.file.read_exact(&mut buf).is_ok() {
                    let next_c = buf[0] as char;
                    if next_c.is_ascii_digit() {
                        number.push(next_c);
                    } else {
                        last_char = Some(next_c);
                        break;
                    }
                }

                //TODO: put check for word bundary

                if let Some(_) = last_char {
                    self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                }

                return Token::NumberU32(self.current_line, number.parse().expect("Expected number"));
            }

            if c == '"' {
                let mut string = String::new();
                while self.file.read_exact(&mut buf).is_ok() {
                    let next_c = buf[0] as char;
                    if next_c == '"' {
                       // println!("String token: {}", string);
                        return Token::StringTok(self.current_line, string);
                    }
                    string.push(next_c);
                }
                //return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Stringa non chiusa"));
            }

            if c.is_ascii_alphabetic() || c == '_' {
                let mut ident = c.to_string();
                while self.file.read_exact(&mut buf).is_ok() && ((buf[0] as char).is_ascii_alphanumeric() || (buf[0] as char) == '_') && (buf[0] as char) != ':' {
                    ident.push(buf[0] as char);
                }

                if buf[0] == b':' {
                    //println!("LABEL: {}", ident);
                    return Token::Label(self.current_line, ident);
                }

                //println!("identifier token: {}", ident);
                return self.keywords.get(&ident).cloned().unwrap_or_else(|| Token::Literal(self.current_line, ident));
            }
            //println!("Single element: {}", buf[0] as char);
            return Token::SingleElem(self.current_line, buf[0] as char);
        }

        Token::EOF(self.current_line)
    }

    pub fn current_line(&self) -> u32 {
        self.current_line
    }
}



