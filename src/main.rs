fn main() {
    println!("Hello, world!");
}

struct Processor {
    memory: Memory,
    tokens: Vec<Token>,
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
            data: Vec::<u8>::new()
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

    fn incr(&mut self) {
        self.data[self.pointer] += 1
    }

    fn decr(&mut self) {
        self.data[self.pointer] -= 1
    }
}
