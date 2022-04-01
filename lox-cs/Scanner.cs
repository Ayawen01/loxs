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
            var c = Advance();
            switch (c)
            {
                case '(':
                    AddToken(TokenType.LEFT_PAREN);
                    break;
                case ')':
                    AddToken(TokenType.RIGHT_PAREN);
                    break;
                case '{':
                    AddToken(TokenType.LEFT_BRACE);
                    break;
                case '}':
                    AddToken(TokenType.RIGHT_BRACE);
                    break;
                case ',':
                    AddToken(TokenType.COMMA);
                    break;
                case '.':
                    AddToken(TokenType.DOT);
                    break;
                case '-':
                    AddToken(TokenType.MINUS);
                    break;
                case '+':
                    AddToken(TokenType.PLUS);
                    break;
                case ';':
                    AddToken(TokenType.SEMICOLON);
                    break;
                case '*':
                    AddToken(TokenType.STAR);
                    break;

                default:
                    break;
            }
        }

        private bool IsAtEnd()
        {
            return Current >= Source.Length;
        }

        private char Advance()
        {
            return Source[Current++];
        }

        private void AddToken(TokenType type)
        {
            AddToken(type, null);
        }

        private void AddToken(TokenType type, object? literal)
        {
            Tokens.Add(new Token(type, literal, Line));
        }
    }
}
