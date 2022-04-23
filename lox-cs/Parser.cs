using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lox_cs
{
    internal class Parser
    {
        private List<Token> Tokens { get; }
        private int Current { get; set; }

        internal Parser(List<Token> tokens)
        {
            Tokens = tokens;
        }

        internal Expr Parse()
        {
            return Expression();
        }

        private Expr Expression()
        {
            return Equality();
        }

        private Expr Equality()
        {
            Expr expr = Comparison();

            while (Match(TokenType.BANG_EQUAL, TokenType.EQUAL_EQUAL))
            {
                Token op = Previous();
                Expr right = Comparison();
                expr = new Expr.Binary(expr, op, right);
            }

            return expr;
        }

        private Expr Comparison()
        {
            Expr expr = Term();

            while (Match(TokenType.GREATER, TokenType.GREATER_EQUAL, TokenType.LESS, TokenType.LESS_EQUAL))
            {
                Token op = Previous();
                Expr right = Term();
                expr = new Expr.Binary(expr, op, right);
            }

            return expr;
        }

        private Expr Term()
        {
            Expr expr = Factor();

            while (Match(TokenType.MINUS, TokenType.PLUS))
            {
                Token op = Previous();
                Expr right = Factor();
                expr = new Expr.Binary(expr, op, right);
            }

            return expr;
        }

        private Expr Factor()
        {
            Expr expr = Unary();

            while (Match(TokenType.SLASH, TokenType.STAR))
            {
                Token op = Previous();
                Expr right = Unary();
                expr = new Expr.Binary(expr, op, right);
            }

            return expr;
        }

        private Expr Unary()
        {
            if (Match(TokenType.BANG, TokenType.MINUS))
            {
                Token op = Previous();
                Expr right = Unary();
                return new Expr.Unary(op, right);
            }

            return Primary();
        }

        private Expr Primary()
        {
            if (Match(TokenType.FALSE)) return new Expr.Literal(false);
            if (Match(TokenType.TRUE)) return new Expr.Literal(true);
            if (Match(TokenType.NIL)) return new Expr.Literal(null);

            if (Match(TokenType.NUMBER, TokenType.STRING))
            {
                return new Expr.Literal(Previous().Literal!);
            }

            if (Match(TokenType.LEFT_PAREN))
            {
                Expr expr = Expression();
                Consume(TokenType.RIGHT_PAREN, "Expect ')' after expression.");
                return new Expr.Grouping(expr);
            }

            throw new ParseError("Expect expression.", Peek().Line);
        }

        private void Synchronize()
        {
            Advance();

            while (!IsAtEnd())
            {
                if (Previous().Type == TokenType.SEMICOLON)
                    return;

                switch (Peek().Type)
                {
                    case TokenType.CLASS:
                    case TokenType.FUN:
                    case TokenType.VAR:
                    case TokenType.FOR:
                    case TokenType.IF:
                    case TokenType.WHILE:
                    case TokenType.PRINT:
                    case TokenType.RETURN:
                        return;
                }

                Advance();
            }
        }

        private Token Consume(TokenType type, string msg)
        {
            if (Check(type))
                return Advance();
            throw new ParseError(msg, Peek().Line);
        }

        private bool Match(params TokenType[] types)
        {
            foreach (TokenType type in types)
            {
                if (Check(type))
                {
                    Advance();
                    return true;
                }
            }

            return false;
        }

        private Token Advance()
        {
            if (!IsAtEnd())
                Current++;
            return Previous();
        }

        private Token Previous()
        {
            return Tokens[Current - 1];
        }

        private bool Check(TokenType type)
        {
            if (IsAtEnd())
                return false;
            return Peek().Type == type;
        }

        private Token Peek()
        {
            return Tokens[Current];
        }

        private bool IsAtEnd()
        {
            return Peek().Type == TokenType.EOF;
        }
    }
}
