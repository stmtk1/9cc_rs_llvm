fn main() -> Result<(), GrammerError> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("引数が{}個あります。引数は2個にしてください", args.len());
        return Err(GrammerError::UnexpectedChar);
    }
    let mut parser = Parser::new(&args[1])?;

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
enum Token {
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

    fn next_token(&mut self) -> Result<Token, GrammerError> {
        if self.pos >= self.input.len() {
            return Ok(Token::End);
        }
        match self.input[self.pos] {
            b'0'..=b'9' => self.lex_number(),
            b'+' => self.lex_plus(),
            b'-' => self.lex_minus(),
            _ => Err(GrammerError::UnexpectedToken)
        }
    }

    fn lex_plus(&mut self) -> Result<Token, GrammerError> {
        self.pos += 1;
        Ok(Token::Plus)
    }

    fn lex_minus(&mut self) -> Result<Token, GrammerError> {
        self.pos += 1;
        Ok(Token::Minus)
    }

    fn lex_number(&mut self) -> Result<Token, GrammerError> {
        let start = self.pos;
        let end = self.recognize_multiple_chars(|b| b"0123456789".contains(&b));
        self.pos = end;
        let number = std::str::from_utf8(&self.input[start..end]).unwrap().parse::<i32>().unwrap();
        Ok(Token::Number(number))
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
    fn new(s: &str) -> Result<Parser, GrammerError> {
        let mut lexer = Lexer::new(s);
        let mut tokens = Vec::new();
        let mut next_token = lexer.next_token();
        loop {
            match &next_token? {
                Token::End => {
                    tokens.push(Token::End);
                    break;
                },
                token => tokens.push(token.clone()),
            }
            next_token = lexer.next_token();
        }
        Ok(Parser {
            tokens,
            pos: 0,
            num: 0,
        })
    }

    fn parse(&mut self) -> Result<(), GrammerError> {
        let a = self.alloc()?;
        self.additonal_ops(a)?;
        Ok(())
    }

    fn alloc(&mut self) -> Result<usize, GrammerError> {
        if let Token::Number(i) = self.tokens[self.pos] {
            println!("\t%{} = alloca i32, align 4", self.num, );
            println!("\tstore i32 {}, i32* %{}, align 4", i, self.num);
            println!("\t%{} = load i32, i32* %{}, align 4", self.num + 1, self.num);
            self.num += 2;
            self.pos += 1;
            Ok(self.num - 1)
        } else {
            Err(GrammerError::UnexpectedToken)
        }
    }

    fn additonal_ops(&mut self, initial_pos: usize) -> Result<usize, GrammerError> {
        let mut add_pos = initial_pos;
        loop {
            match self.tokens[self.pos] {
                Token::Plus => {
                    if let Token::Number(i) = self.tokens[self.pos + 1] {
                        println!("\t%{} = add i32 {}, %{}", self.num, i, add_pos);
                    } else {
                        return Err(GrammerError::UnexpectedToken);
                    }
                },
                Token::Minus => {
                    if let Token::Number(i) = self.tokens[self.pos + 1] {
                        println!("\t%{} = sub i32 {}, %{}", self.num, i, add_pos);
                    } else {
                        return Err(GrammerError::UnexpectedToken);
                    }
                },
                Token::End => {
                    break;
                },
                _ => return Err(GrammerError::UnexpectedToken),
            }
            self.pos += 2;
            self.num += 1;
            add_pos += 1;
        }
        Ok(self.num)
    }
}

#[derive(Debug)]
enum GrammerError {
    UnexpectedToken,
    UnexpectedChar,
    UnexpectedEOL,
}
