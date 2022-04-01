using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lox_cs
{
    internal class Scanner
    {
        private string Source { get; }
        private List<Token> Tokens { get; }
        private int Start { get; set; }
        private int Current { get; set; }
        private int Line { get; set; }

        internal Scanner(string source)
        {
            Source = source;
            Tokens = new List<Token>();
            Start = 0;
            Current = 0;
            Line = 1;
        }

        internal List<Token> ScanTokens()
        {
            while (!IsAtEnd())
            {
                Start = Current;
                ScanToken();
            }

            Tokens.Add(new Token(TokenType.EOF, null, Line));
            return Tokens;
        }

        private void ScanToken()
        {
            throw new NotImplementedException();
        }

        private bool IsAtEnd()
        {
            return Current >= Source.Length;
        }
    }
}
