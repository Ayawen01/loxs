const Token = require("./token");

class Scanner {
    /**
     * 
     * @param {Array<Token>} tokens
     * @param {String} source 
     * @param {Number} start
     * @param {Number} current
     * @param {Number} line
     */
    constructor(source) {
        this.tokens = [];
        this.source = source;
        this.start = 0;
        this.current = 0;
        this.line = 1;
    }

    scan_tokens() {
        while (!this.is_at_end()) {
            this.scan_token();
        }
        return this.tokens;
    }

    scan_token() {

    }

    peek() {
        if (this.is_at_end())
            return '\0';
        return this.source[this.current];
    }

    peek_next() {
        if (this.is_at_end())
            return '\0';
        if (this.current + 1 >= this.source.length)
            return '\0';
        return this.source[this.current + 1];
    }

    advance() {
        return this.source[this.current++];
    }

    /**
     * 
     * @param {String} c 
     */
    match(c) {
        if (this.is_at_end())
            return false;
        if (this.peek() != c)
            return false;
        this.advance();
        return true;
    }
    
    /**
     * 
     * @param {String} c 
     */
    is_alpha_numeric(c) {
        return this.is_alpha(c) || this.is_dight(c);
    }

    /**
     * 
     * @param {String} c 
     */
    is_dight(c) {
        return c >= '0' && c <= '9';
    }

    /**
     * 
     * @param {String} c 
     */
    is_alpha(c) {
        return c >= 'a' && c <= 'z' ||
               c >= 'A' && c <= 'Z' ||
               c == '_';
    }

    is_at_end() {
        return this.current >= this.source.length;
    }

    string() {

    }

    number() {

    }

    identifier() {

    }
}

module.exports = Scanner;