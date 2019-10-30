use std::io::Error;
use std::io::Read;
use std::env::args;
use std::fs::File;

/*
 * The most simplistic and made in a hurry brainfuck interpreter you'll ever see.
 * Might change some stuff, but not now.
*/

// Program struct. It manages the memory, the memory pointer, loop indexes.
// It has a stdin field because I don't like creating a new one each time the command is `,` :))
struct Prog {
    mem: Vec<u8>,
    memi: usize,
    loops: Vec<usize>,
    stdin: std::io::Stdin,
}

impl Prog {
    fn add(&mut self) -> () {
        self.mem[self.memi] += 1;
    }

    fn sub(&mut self) -> () {
        self.mem[self.memi] -= 1;
    }

    fn next(&mut self) -> () {
        self.memi += 1;
        if self.mem.len() < self.memi + 1 {
            self.mem.push(0);
        }
    }

    fn prev(&mut self) -> () {
        self.memi -= 1;
    }

    fn write(&self) -> () {
        println!("{}", self.mem[self.memi] as char);
    }

    fn read(&mut self) -> () {
        let mut input = String::new();
        match self.stdin.read_line(&mut input) {
            Ok(_) => 0,
            Err(_) => {
                println!("ERROR: Unable to read from stdin");
                std::process::exit(0x0001);
            }
        };
        self.mem[self.memi] += input.as_bytes()[0];
    }

    fn add_loop(&mut self, index: usize) -> () {
        self.loops.push(index);
    }

    fn end_loop(&mut self, index: &mut usize) -> () {
        if self.mem[self.memi] == 0 {
            self.loops.pop();
        }
        else {
            let idx = self.loops.len() - 1;
            *index = self.loops[idx];
        }
    }
}

// Constructor for a new Prog type variable
fn get_new_prog() -> Prog {
    Prog {
        mem: vec![0],
        memi: 0,
        loops: vec![],
        stdin: std::io::stdin(),
    }
}

// Struct to represent a brainfuck script
struct BfFile {
    filename: String,
}

impl BfFile {

    // Returns characters from a file
    fn read_file(&mut self) -> Result<Vec<u8>, Error> {
        let mut file = File::open(&self.filename)?;
        let mut temp = String::new();
        file.read_to_string(&mut temp)?;

        Ok(temp.as_bytes().to_vec())
    }

    // Directly executing commands
    fn start(&mut self) -> Result<Vec<u8>, Error> {
        let mut prog = get_new_prog();
        let chars = self.read_file()?;
        let mut i: usize = 0;

        while i < chars.len() {
            let c = chars[i] as char;
            match c {
                '+' => prog.add(),
                '-' => prog.sub(),
                '>' => prog.next(),
                '<' => prog.prev(),
                '.' => prog.write(),
                ',' => prog.read(),
                '[' => prog.add_loop(i),
                ']' => prog.end_loop(&mut i),
                _   => (),
            }
            i += 1;
        }
        Ok(prog.mem)
    }
}

fn load_script(filename: &str) -> BfFile {
    BfFile {
            filename: filename.to_string(),
    }
}

fn main() {
    if let Some(filename) = args().nth(1) {
        let mut script = load_script(&filename);
        println!("{:?}", script.start());
    }
    else {
        println!("No file specified!")
    }
}
