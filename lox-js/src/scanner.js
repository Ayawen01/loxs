const Token = require("./token");
const TokenType = require("./tokentype");

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
        this.tokens.push(new Token(TokenType.Eof, null, this.line));
        return this.tokens;
    }

    scan_token() {
        const c = this.advance();
        switch (c) {
            case '(': this.add_token(TokenType.Left_Paren); break;
            case ')': this.add_token(TokenType.Right_Paren); break;
            case '{': this.add_token(TokenType.Left_Brace); break;
            case '}': this.add_token(TokenType.Right_Brace); break;
            case ',': this.add_token(TokenType.Comma); break;
            case '.': this.add_token(TokenType.Dot); break;
            case '-': this.add_token(TokenType.Minus); break;
            case '+': this.add_token(TokenType.Plus); break;
            case ';': this.add_token(TokenType.Semicolon); break;
            case '*': this.add_token(TokenType.Star); break;

            case '!': this.match('=') ? this.add_token(TokenType.Bang_Equal) : this.add_token(TokenType.Bang); break;
            case '=': this.match('=') ? this.add_token(TokenType.Equal_Equal) : this.add_token(TokenType.Equal); break;
            case '>': this.match('=') ? this.add_token(TokenType.Greater_Equal) : this.add_token(TokenType.Greater); break;
            case '<': this.match('=') ? this.add_token(TokenType.Less_Equal) : this.add_token(TokenType.Less); break;

            case ' ':
            case '\r':
            case '\t': break;
            case '\n': this.line++; break;

            default: break;
        }
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

    /**
     * 
     * @param {TokenType} type
     * @param {Object} literal
     */
    add_token(type, literal = null) {
        this.tokens.push(new Token(type, literal, this.line));
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