function main() {
    const args = process.argv;
    if (args.length == 3) {
        run_file(args[2]);
    } else if (args.length == 2) {
        run_prompt();
    } else {
        console.log("Usage: lox [script]");
    }
}

function run(source) {

}

function run_file(path) {

}

function run_prompt() {

}

main();