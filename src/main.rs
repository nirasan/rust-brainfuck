use std::io::{self, Read, Write};

fn main() {
    println!("Hello, world!");
}

struct Processor {
    memory: Memory,
    pointer: usize,
    tokens: Vec<Token>,
}

impl Processor {
    fn new(tokens: Vec<Token>) -> Processor {
        Processor{
            memory: Memory::new(),
            pointer: 0,
            tokens
        }
    }

    fn process(&mut self) {
        use Token::*;
        loop {
            let result = self.tokens.get(self.pointer);
            if let Some(token) = result {
                match token {
                    NEXT => self.next(),
                    PREV => self.prev(),
                    INCR => self.incr(),
                    DECR => self.decr(),
                    READ => self.read(),
                    WRITE => self.write(),
                    JUMP => self.jump(),
                    BACK => self.back(),
                }
                self.pointer += 1;
            } else {
                break;
            }
        }
    }

    fn next(&mut self) {
        self.memory.next()
    }

    fn prev(&mut self) {
        self.memory.prev()
    }

    fn incr(&mut self) {
        self.memory.incr()
    }

    fn decr(&mut self) {
        self.memory.decr()
    }

    fn read(&mut self) {
        let mut buf = [0];
        io::stdin().read(&mut buf).ok().expect("read error");
        self.memory.set(buf[0]);
    }

    fn write(&mut self) {
        let buf = [self.memory.curr()];
        io::stdout().write(&buf).ok().expect("write error");
    }

    fn jump(&mut self) {
        if !self.memory.is_zero() {
            return;
        }
        use Token::*;
        let mut depth = 0;
        for index in self.pointer..self.tokens.len() {
            let ref token = self.tokens[index];
            match token {
                JUMP => depth += 1,
                BACK => depth -= 1,
                _ => ()
            }
            if depth < 0 {
                self.pointer = index;
                break;
            }
        }
    }

    fn back(&mut self) {
        if self.memory.is_zero() {
            return;
        }
        use Token::*;
        let mut depth = 0;
        for index in (0..self.pointer).rev() {
            let ref token = self.tokens[index];
            match token {
                BACK => depth += 1,
                JUMP => depth -= 1,
                _ => ()
            }
            if depth < 0 {
                self.pointer = index;
                break;
            }
        }
    }
}

#[test]
fn processor_test() {
    let code = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>++++++++[<++++>-]<+.[-]++++++++++.";
    let tokens = Tokenizer::tokenize(code.to_string());
    let mut processor = Processor::new(tokens);
    processor.process();
}

struct Tokenizer {}

impl Tokenizer {
    fn tokenize(input: String) -> Vec<Token> {
        use Token::*;
        let mut tokens = Vec::<Token>::new();
        for c in input.chars() {
            let token = match c {
                '>' => Some(NEXT),
                '<' => Some(PREV),
                '+' => Some(INCR),
                '-' => Some(DECR),
                '.' => Some(WRITE),
                ',' => Some(READ),
                '[' => Some(JUMP),
                ']' => Some(BACK),
                _ => None,
            };
            if let Some(t) = token {
                tokens.push(t);
            }
        }
        return tokens;
    }
}

#[test]
fn tokenizer_test() {
    let input = "><+-.,[]".to_string();
    let tokens = Tokenizer::tokenize(input);
    assert_eq!(tokens.len(), 8);
    println!("{:?}", tokens)
}

#[derive(Debug)]
enum Token {
    NEXT,
    PREV,
    INCR,
    DECR,
    READ,
    WRITE,
    JUMP,
    BACK,
}

struct Memory {
    pointer: usize,
    data: Vec<u8>
}

impl Memory {
    fn new() -> Memory {
        Memory{
            pointer: 0,
            data: vec![0; 1000]
        }
    }

    fn next(&mut self) {
        self.pointer += 1
    }

    fn prev(&mut self) {
        self.pointer -= 1
    }

    fn curr(&mut self) -> u8 {
        self.data[self.pointer]
    }

    fn is_zero(&mut self) -> bool {
        self.data[self.pointer] == 0
    }

    fn set(&mut self, value: u8) {
        self.data[self.pointer] = value
    }

    fn incr(&mut self) {
        self.data[self.pointer] += 1
    }

    fn decr(&mut self) {
        self.data[self.pointer] -= 1
    }
}
