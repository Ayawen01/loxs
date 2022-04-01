using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lox_cs
{
    internal record struct Token(TokenType Type, object? Literal, int Line);
}
