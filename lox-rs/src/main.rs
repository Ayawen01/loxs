use std::{env, process::exit, fs::File, io::{Read, self, Write}};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run(source: Vec<u8>) {
    //let scanner = Scanner::new();
    //let tokens = scanner.scan_tokens();
    //println!("{:?}", tokens);
}

fn run_file(path: &str) -> io::Result<()> {
    let mut source = String::new();

    let mut file = File::open(path)?;

    file.read_to_string(&mut source)?;

    let source = source.into_bytes();

    run(source);

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush()?;

        if let Err(e) = io::stdin().read_line(&mut input) {
            break Err(e)
        }
    
        let source = input.into_bytes();

        run(source);
    }
}