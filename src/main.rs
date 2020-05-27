fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("引数が{}個あります。引数は2個にしてください", args.len());
        return;
    }
    if let Err(err) = parse_main(&args[1]) {
        eprintln!("{}\n{}",args[1] , err);
    }
}

fn parse_main(s: &str) -> Result<(), QccError> {
    let mut parser = Parser::new(s)?;

    println!("define i32 @main() local_unnamed_addr #0 {{");
    println!("entry:");
    parser.parse()?;
    println!("\tret i32 %{}", parser.num - 1);
    println!("}}");
    Ok(())
}

struct Lexer <'a> {
    input: &'a [u8],
    pos: usize,
}

#[derive(Clone, Debug)]
struct Token {
    kind: TokenKind,
    pos: usize,
}

#[derive(Clone, Debug)]
enum TokenKind {
    Number(i32),
    Plus,
    Minus,
    End,
}

impl<'a> Lexer<'a> {
    fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            input: &s.as_bytes(),
            pos: 0
        }
    }

    fn next_token(&mut self) -> Result<Token, QccError> {
        while self.pos < self.input.len() && self.input[self.pos] == b' ' {
            self.pos += 1;
        }
        if self.pos >= self.input.len() {
            return Ok(Token{
                kind: TokenKind::End,
                pos: self.pos,
            });
        }
        match self.input[self.pos] {
            b'0'..=b'9' => self.lex_number(),
            b'+' => self.lex_plus(),
            b'-' => self.lex_minus(),
            _ => Err(QccError{
                kind: ErrorType::UnexpectedChar,
                pos: self.pos,
                message: String::from("unknown expression"),
            })
        }
    }

    fn lex_plus(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token{
            kind: TokenKind::Plus,
            pos: self.pos - 1,
        })
    }

    fn lex_minus(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token{
            kind: TokenKind::Minus,
            pos: self.pos -1,
        })
    }

    fn lex_number(&mut self) -> Result<Token, QccError> {
        let start = self.pos;
        let end = self.recognize_multiple_chars(|b| b"0123456789".contains(&b));
        self.pos = end;
        let num = std::str::from_utf8(&self.input[start..end]).unwrap().parse::<i32>().unwrap();
        Ok(Token{
            kind: TokenKind::Number(num),
            pos: start,
        })
    }

    fn recognize_multiple_chars(&self, mut f: impl FnMut(u8) -> bool) -> usize {
        let mut pos = self.pos;

        while pos < self.input.len() && f(self.input[pos]) {
            pos += 1;
        }
        pos
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    num: usize,
}

impl Parser {
    fn new(s: &str) -> Result<Parser, QccError> {
        let mut lexer = Lexer::new(s);
        let mut tokens = Vec::new();
        let mut next_token = lexer.next_token()?;
        loop {
            match &next_token.kind {
                TokenKind::End => {
                    tokens.push(next_token.clone());
                    break;
                },
                token => tokens.push(next_token.clone()),
            }
            next_token = lexer.next_token()?;
        }
        Ok(Parser {
            tokens,
            pos: 0,
            num: 0,
        })
    }

    fn parse(&mut self) -> Result<(), QccError> {
        let a = self.alloc()?;
        self.multiplitive_ops(a)?;
        Ok(())
    }

    fn alloc(&mut self) -> Result<usize, QccError> {
        if let TokenKind::Number(i) = self.tokens[self.pos].kind {
            println!("\t%{} = alloca i32, align 4", self.num);
            println!("\tstore i32 {}, i32* %{}, align 4", i, self.num);
            println!("\t%{} = load i32, i32* %{}, align 4", self.num + 1, self.num);
            self.num += 2;
            self.pos += 1;
            Ok(self.num - 1)
        } else {
            Err(QccError{
                kind: ErrorType::UnexpectedToken,
                pos: self.tokens[self.pos].pos,
                message: String::from("This expression expected number"),
            })
        }
    }

    fn additonal_ops(&mut self, initial_pos: usize) -> Result<usize, QccError> {
        let mut add_pos = initial_pos;
        loop {
            match self.tokens[self.pos].kind {
                TokenKind::Plus => {
                    if let TokenKind::Number(i) = self.tokens[self.pos + 1].kind {
                        println!("\t%{} = add i32 {}, %{}", self.num, i, add_pos);
                    } else {
                        return Err(QccError {
                            kind: ErrorType::UnexpectedToken,
                            pos: self.tokens[self.pos].pos,
                            message: String::from("This expression expected number"),
                        });
                    }
                },
                TokenKind::Minus => {
                    if let TokenKind::Number(i) = self.tokens[self.pos + 1].kind {
                        println!("\t%{} = sub i32 {}, %{}", self.num, i, add_pos);
                    } else {
                        return Err(QccError{
                            kind: ErrorType::UnexpectedToken,
                            pos: self.tokens[self.pos].pos,
                            message: String::from("This expression expected number"),
                        });
                    }
                },
                TokenKind::End => {
                    break;
                },
                _ => return Err(QccError{
                    kind: ErrorType::UnexpectedToken,
                    pos: self.tokens[self.pos].pos,
                    message: String::from("This expression expected + or -"),
                }),
            }
            self.pos += 2;
            self.num += 1;
            add_pos += 1;
        }
        Ok(self.num)
    }

    fn multiplitive_ops(&mut self, initial_pos: usize) -> Result<usize, QccError> {
        self.additonal_ops(initial_pos)
    }
}

#[derive(Debug)]
enum ErrorType {
    UnexpectedToken,
    UnexpectedChar,
    UnexpectedEOL,
}

#[derive(Debug)]
struct QccError {
    kind: ErrorType,
    pos: usize,
    message: String,
}


impl std::fmt::Display for QccError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for _ in 0..self.pos {
            write!(f, " ")?;
        }
        write!(f, "^\n{}", self.message)
    }
}
