const readline = require('readline');
const fs = require('fs');

function main() {
    const args = process.argv;
    if (args.length == 3) {
        run_file(args[2]);
    } else if (args.length == 2) {
        run_prompt();
    } else {
        console.log('Usage: lox [script]');
    }
}

function run(source) {
    console.log(source);
}

function run_file(path) {
    const source = fs.readFileSync(path);
    run(source.toString());
}

function run_prompt() {
    readline.createInterface({
        input: process.stdin,
        output: process.stdout,
        prompt: '> '
    }).on('line', line => {
        run(line);
    });
}

main();