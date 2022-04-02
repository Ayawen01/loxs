const TokenType = {
    // Single-character tokens.
    Left_Paren:'Left_Paren', Right_Paren:'Right_Paren', Left_Brace:'Left_Brace', Right_Brace:'Right_Brace',
    Comma:'Comma', Dot:'Dot', Minus:'Minus', Plus:'Plus', Semicolon:'Semicolon', Slash:'Slash', Star:'Star',

    // One or two character tokens.
    Bang:'Bang', Bang_Equal:'Bang_Equal',
    Equal:'Equal', Equal_Equal:'Equal_Equal',
    Greater:'Greater', Greater_Equal:'Greater_Equal',
    Less:'Less', Less_Equal:'Less_Equal',

    // Literals.
    Identifier:'Identifier', String:'String', Number:'Number',

    // Keywords.
    And:'And', Class:'Class', Else:'Else', False:'False', Fun:'Fun', For:'For', If:'If', Nil:'Nil', Or:'Or',
    Print:'Print', Return:'Return', Super:'Super', This:'This', True:'True', Var:'Var', While:'While',

    Eof:'Eof'
};

module.exports = TokenType;