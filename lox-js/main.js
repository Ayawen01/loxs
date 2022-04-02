function main() {
    const args = Deno.args;
    if (args.length < 2) {
        run_file(args[0]);
    } else if (args.length == 1) {
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