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
        internal List<Token> Tokens { get; }
        private int Start { get; set; }
        private int Current { get; set; }
        private int Line { get; set; }

        private static Dictionary<string, TokenType> KeyWords = new Dictionary<string, TokenType>()
        {
            { "and",    TokenType.AND },
            { "class",  TokenType.CLASS },
            { "else",   TokenType.ELSE },
            { "false",  TokenType.FALSE },
            { "for",    TokenType.FOR },
            { "fun",    TokenType.FUN },
            { "if",     TokenType.IF },
            { "nil",    TokenType.NIL },
            { "or",     TokenType.OR },
            { "print",  TokenType.PRINT },
            { "return", TokenType.RETURN },
            { "super",  TokenType.SUPER },
            { "this",   TokenType.THIS },
            { "true",   TokenType.TRUE },
            { "var",    TokenType.VAR },
            { "while",  TokenType.WHILE }
        };

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

                case '!':
                    AddToken(Match('=') ? TokenType.BANG_EQUAL : TokenType.BANG);
                    break;
                case '=':
                    AddToken(Match('=') ? TokenType.EQUAL_EQUAL : TokenType.EQUAL);
                    break;
                case '>':
                    AddToken(Match('=') ? TokenType.GREATER_EQUAL : TokenType.GREATER);
                    break;
                case '<':
                    AddToken(Match('=') ? TokenType.LESS_EQUAL : TokenType.LESS);
                    break;

                case '/':
                    if (Match('/'))
                        while (Peek() != '\n' && !IsAtEnd())
                            Advance();
                    else
                        AddToken(TokenType.SLASH);
                    break;

                case ' ':
                case '\r':
                case '\t':
                    break;

                case '\n':
                    Line++;
                    break;

                case '"':
                    ScanString();
                    break;

                default:
                    if (IsDight(c))
                    {
                        ScanNumber();
                    }
                    else if (IsAlpha(c))
                    {
                        ScanIdentifier();
                    }
                    else
                    {
                        throw new LexError("未知的词素.", Line, c);
                    }
                    break;
            }
        }

        private void ScanString()
        {
            Start = Current;
            while (Peek() != '"' && !IsAtEnd())
            {
                if (Peek() == '\n')
                    Line++;
                Advance();
            }

            if (IsAtEnd())
                throw new LexError("不是一串完整的字符串.", Line);

            var str = Source.Substring(Start, Current - Start);
            AddToken(TokenType.STRING, str);

            Advance();
        }

        private void ScanNumber()
        {
            Start = Current - 1;
            while (IsDight(Peek()))
                Advance();
            
            if (Peek() == '.' && !IsAtEnd())
            {
                Advance();

                while (IsDight(Peek()))
                    Advance();
            }

            double number = Convert.ToDouble(Source.Substring(Start, Current - Start));
            AddToken(TokenType.NUMBER, number);
        }

        private void ScanIdentifier()
        {
            Start = Current - 1;
            while (IsAlphaNumeric(Peek()))
                Advance();

            string id = Source.Substring(Start, Current - Start);
            if (KeyWords.ContainsKey(id))
            {
                AddToken(KeyWords[id]);
                return;
            }
            AddToken(TokenType.IDENTIFIER, id);
        }

        private bool IsAtEnd()
        {
            return Current >= Source.Length;
        }

        private bool IsAlphaNumeric(char c)
        {
            return IsAlpha(c) || IsDight(c);
        }

        private bool IsAlpha(char c)
        {
            return c >= 'a' && c <= 'z' ||
                   c >= 'A' && c <= 'Z' ||
                   c == '_';
        }

        private bool IsDight(char c)
        {
            return (c >= '0' && c <= '9');
        }

        private char Advance()
        {
            return Source[Current++];
        }

        private bool Match(char c)
        {
            if (IsAtEnd())
                return false;
            if (Peek() != c)
                return false;
            Current++;
            return true;
        }

        private char Peek()
        {
            if (IsAtEnd())
                return '\0';
            return Source[Current];
        }

        private char PeekNext()
        {
            if (Current + 1 >= Source.Length)
                return '\0';
            return Source[Current + 1];
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
