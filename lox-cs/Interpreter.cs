using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lox_cs
{
    internal class Interpreter : Expr.IVisitor<object>
    {
        internal void Interpret(Expr expr)
        {
            object value = Evaluate(expr);
            Console.WriteLine(Stringify(value));
        }

        private string Stringify(object obj)
        {
            if (obj is null)
                return "nil";

            if (obj is double)
            {
                string text = obj.ToString()!;
                if (text.EndsWith(".0"))
                {
                    text = text.Substring(0, text.Length - 2);
                }
                return text;
            }

            return obj.ToString();
        }

        private object Evaluate(Expr expression)
        {
            return expression.Accept(this);
        }

        private bool IsTruthy(object obj)
        {
            if (obj is null)
                return false;
            if (obj is bool boolean)
                return boolean;
            return true;
        }

        public object VisitAssignExpr(Expr.Assign expr)
        {
            throw new NotImplementedException();
        }

        public object VisitBinaryExpr(Expr.Binary expr)
        {
            object left = Evaluate(expr.Left);
            object right = Evaluate(expr.Right);

            switch (expr.Operator.Type) {
                case TokenType.BANG_EQUAL:
                    return !IsEqual(left, right);
                case TokenType.EQUAL_EQUAL:
                    return IsEqual(left, right);
                case TokenType.GREATER:
                    CheckNumberOperands(expr.Operator, left, right);
                    return (double)left > (double)right;
                case TokenType.GREATER_EQUAL:
                    CheckNumberOperands(expr.Operator, left, right);
                    return (double)left >= (double)right;
                case TokenType.LESS:
                    CheckNumberOperands(expr.Operator, left, right);
                    return (double)left < (double)right;
                case TokenType.LESS_EQUAL:
                    CheckNumberOperands(expr.Operator, left, right);
                    return (double)left <= (double)right;
                case TokenType.MINUS:
                    CheckNumberOperands(expr.Operator, left, right);
                    return (double)left - (double)right;
                case TokenType.SLASH:
                    CheckNumberOperands(expr.Operator, left, right);
                    return (double)left / (double)right;
                case TokenType.STAR:
                    CheckNumberOperands(expr.Operator, left, right);
                    return (double)left * (double)right;
                case TokenType.PLUS:
                    if (left is double && right is double)
                        return (double)left + (double)right;
                    if (left is string && right is string)
                        return (string)left + (string)right;
                    throw new RuntimeError("Operands must be two numbers or two strings.", expr.Operator.Line);
            }

            return null;
        }

        private bool IsEqual(object left, object right)
        {
            if (left is null && right is null)
                return true;
            if (left is null)
                return false;

            return left == right;
        }

        public object VisitCallExpr(Expr.Call expr)
        {
            throw new NotImplementedException();
        }

        public object VisitGetExpr(Expr.Get expr)
        {
            throw new NotImplementedException();
        }

        public object VisitGroupingExpr(Expr.Grouping expr)
        {
            return Evaluate(expr.Expression);
        }

        public object VisitLiteralExpr(Expr.Literal expr)
        {
            return expr.Value;
        }

        public object VisitLogicalExpr(Expr.Logical expr)
        {
            throw new NotImplementedException();
        }

        public object VisitSetExpr(Expr.Set expr)
        {
            throw new NotImplementedException();
        }

        public object VisitSuperExpr(Expr.Super expr)
        {
            throw new NotImplementedException();
        }

        public object VisitThisExpr(Expr.This expr)
        {
            throw new NotImplementedException();
        }

        public object VisitUnaryExpr(Expr.Unary expr)
        {
            object right = Evaluate(expr.Right);

            switch (expr.Operator.Type)
            {
                case TokenType.MINUS:
                    CheckNumberOperand(expr.Operator, right);
                    return -(double)right;
                case TokenType.BANG:
                    return !IsTruthy(right);
            }

            return null;
        }

        private void CheckNumberOperand(Token @operator, object operand)
        {
            if (operand is double)
                return;
            throw new RuntimeError("Operand must be a number.", @operator.Line);
        }

        private void CheckNumberOperands(Token @operator, object left, object right)
        {
            if (left is double && right is double)
                return;
            throw new RuntimeError("Operand must be a number.", @operator.Line);
        }

        public object VisitVariableExpr(Expr.Variable expr)
        {
            throw new NotImplementedException();
        }
    }
}
