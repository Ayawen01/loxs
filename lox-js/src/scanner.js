const Token = require('./token');
const TokenType = require('./tokentype');
const { LexError } = require('./error');

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

            case '/':
                if (this.match('/'))
                    while (this.peek() != '\n' && !this.is_at_end())
                        this.advance();
                else
                    this.add_token(TokenType.Slash);

            case ' ':
            case '\r':
            case '\t': break;
            case '\n': this.line++; break;

            case '"': this.string(); break;
                

            default:
                if (this.is_dight(c))
                    this.number();
                else if (this.is_alpha(c))
                    this.identifier();
                else
                    throw new LexError('未知的词素.', this.line, c);
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
        this.start = this.current;
        
        while (this.peek() != '"' && !this.is_at_end()) {
            if (this.peek() == '\n')
                this.line++;
            this.advance();
        }

        if (this.is_at_end())
            throw new LexError('不是一串完整的字符串.', this.line);

        const str = this.source.slice(this.start, this.current);
        this.add_token(TokenType.String, str);
        this.advance();
    }

    number() {
        this.start = this.current - 1;

        while (this.is_dight(this.peek()))
            this.advance();
        
        if (this.peek() == '.' && !this.is_at_end()) {
            this.advance();

            while (this.is_dight(this.peek()))
                this.advance();
        }

        const number = Number(this.source.slice(this.start, this.current));
        this.add_token(TokenType.Number, number);
    }

    identifier() {
        this.start = this.current - 1;

        while (this.is_alpha_numeric(this.peek()) && !this.is_at_end())
            this.advance();
        
        const id = this.source.slice(this.start, this.current);

        switch (id) {
            case 'and'   : this.add_token(TokenType.And); break;
            case 'class' : this.add_token(TokenType.Class); break;
            case 'else'  : this.add_token(TokenType.Else); break;
            case 'false' : this.add_token(TokenType.False); break;
            case 'for'   : this.add_token(TokenType.For); break;
            case 'fun'   : this.add_token(TokenType.Fun); break;
            case 'if'    : this.add_token(TokenType.If); break;
            case 'nil'   : this.add_token(TokenType.Nil); break;
            case 'or'    : this.add_token(TokenType.Or); break;
            case 'print' : this.add_token(TokenType.Print); break;
            case 'return': this.add_token(TokenType.Return); break;
            case 'super' : this.add_token(TokenType.Super); break;
            case 'this'  : this.add_token(TokenType.This); break;
            case 'true'  : this.add_token(TokenType.True); break;
            case 'var'   : this.add_token(TokenType.Var); break;
            case 'while' : this.add_token(TokenType.While); break;
            default      : this.add_token(TokenType.Identifier, id);
        }
    }
}

module.exports = Scanner;