using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lox_cs
{
    internal struct Token
    {
        internal TokenType Type { get; }
        internal object? Literal { get; }
        internal int Line { get; }

        internal Token(TokenType type, object? literal, int line)
        {
            Type = type;
            Literal = literal;
            Line = line;
        }

        public override string ToString() => string.Format("Token {{ Type: {0}, Literal: {1}, Line: {2} }}", Type, Literal, Line);
    }
}
