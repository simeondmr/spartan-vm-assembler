use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Mutex, OnceLock};
use regex::Regex;

pub static LEXER_SINGLETON: OnceLock<Mutex<Lexer>> = OnceLock::new();


const SECTION_BSS: &str = "^section_bss";
const SECTION_DATA: &str = "^section_data";
const SECTION_TEXT: &str = "^section_text";
const IADDB: &str = "^iaddb";

const RESB: &str = "^resb";
const RESW: &str = "^resw";
const RESD: &str = "^resd";

const PUSHB: &str = "^pushb";

#[derive(Debug)]
#[derive(Clone)]
#[allow(dead_code)]
pub enum Token {
    Iaddb(u32),
    Pushb(u32),
    Jmp(u32),
    Hlt(u32),
    SectionBssTok(u32),
    SectionData(u32),
    SectionText(u32),
    RESB(u32),
    RESW(u32),
    RESD(u32),
    NumberU32(u32, u32),
    NumberI32(u32, i32),
    Literal(u32, String),
    Label(u32, String),
    CharTok(u32, char),
    StringTok(u32, String),
    SingleElem(u32, char),
    Comment,
    EOF(u32),
    INIT(u32),
    Unknown(u32, String)
}

impl Token {
    pub fn line(&self) -> u32 {
        match self {
            Token::Iaddb(line) => *line,
            Token::Pushb(line) => *line,
            Token::Hlt(line) => *line,
            Token::Jmp(line) => *line,
            Token::SectionBssTok(line) => *line,
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
            _ => {0}
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Iaddb(_), Token::Iaddb(_)) => true,
            (Token::Pushb(_), Token::Pushb(_)) => true,
            (Token::Jmp(_), Token::Jmp(_)) => true,
            (Token::NumberU32(_, _), Token::NumberU32(_, _)) => true,
            (Token::Literal(_, _), Token::Literal(_, _)) => true,
            (Token::CharTok(_, _), Token::CharTok(_, _)) => true,
            (Token::Label(_, _), Token::Label(_, _)) => true,
            (Token::StringTok(_, _), Token::StringTok(_, _)) => true,
            (Token::SingleElem(_, element0), Token::SingleElem(_, element1)) => if element0 == element1{ true } else { false },
            (Token::SectionBssTok(_), Token::SectionBssTok(_)) => true,
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
    reader: BufReader<File>,
    buffer: String,
    current_token: Token,
    current_line: u32
}

impl Lexer {
    pub fn new(file_name: String) -> Self {
        let file = File::open(file_name).expect("Unable to open file");
        Lexer {
            reader: BufReader::new(file),
            buffer: String::new(),
            current_token: Token::INIT(0),
            current_line: 0
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            if self.buffer.is_empty() {
                let mut line = String::new();
                if self.reader.read_line(&mut line).unwrap() == 0 {

                    self.current_token = Token::EOF(self.current_line);
                    return self.current_token.clone() // Fine del file
                }

                self.current_line += 1;
                self.buffer = line;
            }

            let input = self.buffer.as_str();

            if let Some(mat) = Regex::new(r"^\s+").unwrap().find(input) {
                self.buffer = self.buffer[mat.end()..].to_string();
                continue;
            }

            break;
        }


        let input = self.buffer.as_str();

        let patterns = vec![
            (r#"^"(?:[^"\\]|\\.)*""#, Token::StringTok(0, "".to_string())),
            (r#"^'([^'\\]|\\.)'"#, Token::CharTok(0, ' ')),
            (SECTION_BSS, Token::SectionBssTok(0)),
            (SECTION_DATA, Token::SectionData(0)),
            (SECTION_TEXT, Token::SectionText(0)),
            (RESB, Token::RESB(0)),
            (RESW, Token::RESW(0)),
            (RESD, Token::RESD(0)),
            (IADDB, Token::Iaddb(0)),
            (PUSHB, Token::Pushb(0)),
            (r"^[a-zA-Z_][a-zA-Z0-9_]*:", Token::Label(0, "".to_string())),
            (r"^[a-zA-Z_][a-zA-Z0-9_]*", Token::Literal(0, "".to_string())),
            (r"^[+]?\d+", Token::NumberU32(0,0)),
            (r"^[-]?\d+", Token::NumberI32(0,0)),
            (r"^//.*", Token::Comment),
            (r"^[\+\-\*/=;(){}\[\]\,]", Token::SingleElem(0, ' ')),
        ];

        for (pattern, token_type) in &patterns {
            if let Some(mat) = Regex::new(pattern).unwrap().find(input) {
                let matched_str = mat.as_str().to_string();
                self.buffer = self.buffer[mat.end()..].to_string();

                let token = match token_type {
                    Token::SectionBssTok(_) => Token::SectionBssTok(self.current_line),
                    Token::SectionData(_) => Token::SectionData(self.current_line),
                    Token::SectionText(_) => Token::SectionText(self.current_line),
                    Token::RESB(_) => Token::RESB(self.current_line),
                    Token::RESW(_) => Token::RESB(self.current_line),
                    Token::RESD(_) => Token::RESB(self.current_line),
                    Token::Iaddb(_) => Token::Iaddb(self.current_line),
                    Token::Pushb(_) => Token::Pushb(self.current_line),
                    Token::StringTok(_, _) => Token::StringTok(self.current_line, matched_str.trim_matches('"').to_string()),
                    Token::Literal(_, _) => Token::Literal(self.current_line, matched_str),
                    Token::NumberU32(_, _) => Token::NumberU32(self.current_line, matched_str.parse().expect("expected number")),
                    Token::NumberI32(_, _) => Token::NumberI32(self.current_line, matched_str.parse().expect("expected number")),
                    Token::Label(_, _) => Token::Label(self.current_line, matched_str),
                    Token::Comment => Token::Comment,
                    Token::SingleElem(_, _) => Token::SingleElem(self.current_line, matched_str.chars().next().unwrap()),
                    Token::CharTok(_, _) => Token::CharTok(self.current_line, matched_str.chars().nth(1).unwrap()),
                    _ => Token::Unknown(self.current_line, matched_str),
                };

                self.current_token = token.clone();

                return token;
            }
        }

        let unknown_char = input.chars().next().unwrap().to_string();
        self.buffer = self.buffer[1..].to_string();
        let unknow = Token::Unknown(self.current_line, unknown_char);
        self.current_token = unknow.clone();
        unknow
    }

    pub fn is_instruction(&self) -> bool {
        match self.current_token {
            Token::Iaddb(_) => true,
            Token::Pushb(_) => true,
            Token::Jmp(_) => true,
            _ => false
        }
    }

    pub fn current_token(&self) -> Token {
        self.current_token.clone()
    }

    pub fn current_line(&self) -> u32 {
        self.current_line
    }
}