use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Token {
    Identifier(String),
    Number(String),
    String(String),
    Colon,
    Semicolon,
    Comma,
    LeftBracket,
    RightBracket,
}

fn token_line(line: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut i = 0;
    while i < line.len() {
        while line.chars().nth(i).unwrap().is_whitespace() { // skip over any whitespace
            i += 1;
        }

        if line.chars().nth(i).unwrap_or('0').is_alphabetic() ||
            line.chars().nth(i) == Some('.') { // 0 surely isn't alphabetic, right?
            let mut ident = String::new();

            while let Some(c) = line.chars().nth(i) {
                if !c.is_alphanumeric() && c != '.' { break; }
                ident.push(c);
                i += 1;
            }

            tokens.push(Token::Identifier(ident));
            continue;
        }
        if line.chars().nth(i).unwrap_or('a').is_numeric() { // a surely isn't numeric, right?
            let mut num = String::new();

            while let Some(c) = line.chars().nth(i) {
                if !c.is_numeric() { break; }
                num.push(c);
                i += 1;
            }

            tokens.push(Token::Number(num));
            continue;
        }
        if line.chars().nth(i) == Some('\"') {
            let mut str = String::new();

            i += 1;
            while let Some(c) = line.chars().nth(i) {
                i += 1;
                if c == '\"' { break; }
                str.push(c);
            }

            tokens.push(Token::String(str));
            continue;
        }

        match line.chars().nth(i).unwrap() {
            ':' => {
                tokens.push(Token::Colon);
                i += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                i += 1;
            }
            ']' => {
                tokens.push(Token::RightBracket);
                i += 1;
            }
            x => {
                panic!("Unknown character: {x}");
            }
        }
    }

    tokens
}

#[derive(Debug, Clone, Copy)]
pub enum CpuFlag { // u8
    Halt        = 0x01,
    Overflow    = 0x02,
    Negative    = 0x04,
    Zero        = 0x08,
    Carry       = 0x10,
}

struct Labeler {
    labels: HashMap<String, u64>,
    location: u64,
}

impl Labeler {
    pub fn new() -> Self {
        Labeler {
            labels: HashMap::new(),
            location: 0,
        }
    }

    pub fn create_label(&mut self, name: String) {
        self.labels.insert(name, self.location);
    }
    pub fn get_label(&self, name: String) -> u64 {
        *self.labels.get(&name).unwrap()
    }
}

fn parse_tokens(tokens: Vec<Token>, labeler: &mut Labeler) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    fn reg_id(reg: String) -> Option<u8> {
        match reg.as_str() {
            "rax" => Some(0),
            "rbx" => Some(1),
            "rcx" => Some(2),
            "rdx" => Some(3),

            "rbp" => Some(4),
            "rsp" => Some(5),
            "rip" => Some(6),
            _ => None,
        }
    }

    let mut i = 0;
    'line_loop: while i < tokens.len() {
        match tokens.get(i).unwrap() {
            Token::Identifier(x) => {
                if tokens.get(i + 1) == Some(&Token::Colon) {
                    labeler.create_label(x.to_owned());
                    break 'line_loop;
                }

                let mut inst_bytes: Vec<u8> = match x.as_str() {
                    "nop" => {
                        i += 1;
                        vec![0x00]
                    }
                    "hlt" => {
                        i += 1;
                        vec![0x01]
                    }
                    // "add" => 0x03,
                    // "sub" => 0x04,
                    // "cmp" => 0x09,
                    "mov" | "add" | "sub" | "cmp" => {
                        let mut inst = vec![
                            match x.as_str() {
                                "mov" => 0x02,
                                "add" => 0x03,
                                "sub" => 0x04,
                                "cmp" => 0x09,
                                _ => unreachable!(""),
                            }
                        ];
                        let mut path = 0x00u8;
                        let mut regs = 0x00u8;
                        let mut imms = Vec::new();

                        i += 1; // double operand instruction
                        match tokens.get(i).unwrap() {
                            Token::Identifier(reg) => { // register
                                path |= 0b0100;
                                regs |= reg_id(reg.to_owned()).unwrap();
                            }
                            Token::LeftBracket => { // memory address
                                path |= 0b0001;
                                i += 1;
                                match tokens.get(i).unwrap() {
                                    Token::Identifier(reg) => { // register
                                        path |= 0b0100;
                                        regs |= reg_id(reg.to_owned()).unwrap();
                                    }
                                    Token::Number(num) => {
                                        let val = u64::from_str_radix(num, 10).unwrap();
                                        imms.push(val);
                                    }
                                    x => panic!("memory destination cannot be {x:?}"),
                                }
                            }
                            x => panic!("destination cannot be {x:?}"),
                        }
                        i += 2; // comma
                        match tokens.get(i).unwrap() {
                            Token::Identifier(reg) => { // register
                                path |= 0b1000;
                                regs |= reg_id(reg.to_owned()).unwrap() << 4;
                            }
                            Token::Number(num) => { // register
                                let val = u64::from_str_radix(num, 10).unwrap();
                                imms.push(val);
                            }
                            Token::LeftBracket => { // memory address
                                path |= 0b0010;
                                i += 1;
                                match tokens.get(i).unwrap() {
                                    Token::Identifier(reg) => { // register
                                        path |= 0b1000;
                                        regs |= reg_id(reg.to_owned()).unwrap() << 4;
                                    }
                                    Token::Number(num) => {
                                        let val = u64::from_str_radix(num, 10).unwrap();
                                        imms.push(val);
                                    }
                                    x => panic!("memory source cannot be {x:?}"),
                                }
                            }
                            x => panic!("source cannot be {x:?}"),
                        }
                        i += 1;

                        inst.push(path);
                        inst.push(regs);

                        fn num_arr(num: u64) -> Vec<u8> {
                            let mut arr = Vec::new();
                            for i in (0..size_of::<u64>()).rev() {
                                arr.push((num >> (8 * i)) as u8);
                            }
                            arr
                        }
                        for imm in imms {
                            let mut imm_arr = num_arr(imm);
                            inst.append(&mut imm_arr);
                        }

                        // if x == "sub" || x == "add" {
                        //     eprintln!("ar: {inst:?}");
                        // }

                        inst
                    }
                    // "inv" => 0x06,
                    // "jmp" => 0x07,
                    // "je" | "jne" => 0x08,
                    "not" | "inv" | "jmp" | "je" | "jne" | "jz"| "jnz" => {
                        let mut inst = vec![
                            match x.as_str() {
                                "not" => 0x05,
                                "inv" => 0x06,
                                "jmp" => 0x07,
                                "je"  => 0x08,
                                "jne" => 0x08,
                                "jz"  => 0x08,
                                "jnz" => 0x08,
                                _ => unreachable!(""),
                            }
                        ];
                        let mut meta = 0x00u8;
                        let mut imms = Vec::new();

                        if vec![
                            "jne", // idk add more at some point
                            "jnz",
                        ].contains(&x.as_str()) {
                            meta |= 0x80u8;
                        }

                        i += 1; // single operand instruction
                        match tokens.get(i).unwrap() {
                            Token::Identifier(reg) => { // register
                                if let Some(id) = reg_id(reg.to_owned()) {
                                    meta |= 0b100000;
                                    meta |= id;
                                } else {
                                    let loc = labeler.get_label(reg.to_owned());
                                    imms.push((8, loc));
                                }
                            }
                            Token::LeftBracket => { // memory address
                                meta |= 0b010000;
                                i += 1;
                                match tokens.get(i).unwrap() {
                                    Token::Identifier(reg) => { // register
                                        meta |= 0b100000;
                                        if let Some(id) = reg_id(reg.to_owned()) {
                                            meta |= id;
                                        } else {
                                            labeler.create_label(reg.to_owned());
                                        }
                                    }
                                    Token::Number(num) => {
                                        let val = u64::from_str_radix(num, 10).unwrap();
                                        imms.push((8, val));
                                    }
                                    x => panic!("memory destination cannot be {x:?}"),
                                }
                            }
                            Token::Number(num) => {
                                let val = u64::from_str_radix(num, 10).unwrap();
                                imms.push((8, val));
                            }
                            y => panic!("destination cannot be {y:?} - {x:?}"),
                        }
                        i += 1;

                        match x.as_str() {
                            "je" => {
                                // imms.insert(0, (1, CpuFlag::Zero as u64));
                                inst.push(CpuFlag::Zero as u8);
                            }
                            "jne" => {
                                // imms.insert(0, (1, CpuFlag::Zero as u64));
                                inst.push(CpuFlag::Zero as u8);
                                meta |= 0b10000000u8;
                            }
                            "jz" => {
                                // imms.insert(0, (1, CpuFlag::Zero as u64));
                                inst.push(CpuFlag::Zero as u8);
                            }
                            "jnz" => {
                                // imms.insert(0, (1, CpuFlag::Zero as u64));
                                inst.push(CpuFlag::Zero as u8);
                                meta |= 0b10000000u8;
                            }
                            _ => {},
                        }
                        inst.push(meta);

                        fn num_arr(num: (u64, u64)) -> Vec<u8> { // (bytes, number)
                            let mut arr = Vec::new();
                            for i in (0..num.0).rev() {
                                arr.push((num.1 >> (8 * i)) as u8);
                            }
                            arr
                        }
                        for imm in imms {
                            let mut imm_arr = num_arr(imm);
                            inst.append(&mut imm_arr);
                        }

                        // eprintln!("jc bytes: {inst:?}");

                        inst
                    }
                    x => panic!("unknown instruction: {x}"),
                };
                bytes.append(&mut inst_bytes);
            }
            Token::Number(_) => panic!("instruction cannot just be a number"),
            Token::String(_) => panic!("instruction cannot just be a string"),
            Token::Colon => panic!("instruction cannot just be a colon"),
            Token::Comma => panic!("instruction cannot just be a comma"),
            Token::LeftBracket => panic!("instruction cannot just be a left bracket"),
            Token::RightBracket => panic!("instruction cannot just be a right bracket"),
            Token::Semicolon => {
                break 'line_loop;
            }
        }
    }

    bytes
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("no input file given");
    let output = args.get(2).expect("no output file given");

    let contents = std::fs::read_to_string(input).expect("could not find input file");
    let lines: Vec<String> = contents.lines().map(|x| x.to_owned()).collect();

    let mut label_table: Labeler = Labeler::new();
    let mut prog_bytes: Vec<u8> = Vec::new();

    for line in lines {
        let tokens = token_line(line);
        let bytes = parse_tokens(tokens.clone(), &mut label_table);
        label_table.location += bytes.len() as u64;
        bytes.into_iter().for_each(|x| {
            prog_bytes.push(x);
        });
    }

    std::fs::write(output, prog_bytes).expect("could not write to output file");
}
