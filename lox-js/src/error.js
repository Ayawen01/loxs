class LexError extends Error {
    /**
     * 
     * @param {String} message 
     * @param {Number} line 
     * @param {String} c 
     */
    constructor(message, line, c = null) {
        super(message);
        this.c = c;
        this.line = line;
    }

    toString() {
        if (this.c == null)
            return `[line ${this.line}] LexError ${this.message}`;
        return `[line ${this.line}] LexError \`${this.c}\` ${this.message}`;
    }
}

module.exports = { LexError };