const TokenType = require('./tokentype');

class Token {
    /**
     * @param {TokenType} type 
     * @param {Object} literal 
     * @param {Number} line 
     */
    constructor(type, literal, line) {
        this.type = type;
        this.literal = literal;
        this.line = line;
    }
}

module.exports = Token;