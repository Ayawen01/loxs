using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lox_cs
{
    internal abstract class Expr
    {
        internal interface IVisitor<R>
        {
            R VisitAssignExpr(Assign expr);
            R VisitBinaryExpr(Binary expr);
            R VisitCallExpr(Call expr);
            R VisitGetExpr(Get expr);
            R VisitGroupingExpr(Grouping expr);
            R VisitLiteralExpr(Literal expr);
            R VisitLogicalExpr(Logical expr);
            R VisitSetExpr(Set expr);
            R VisitSuperExpr(Super expr);
            R VisitThisExpr(This expr);
            R VisitUnaryExpr(Unary expr);
            R VisitVariableExpr(Variable expr);
        }

        internal abstract R Accept<R>(IVisitor<R> visitor);

        internal class Assign : Expr
        {
            internal Token Name { get; }
            internal Expr Value { get; }

            internal Assign(Token name, Expr value)
            {
                Name = name;
                Value = value;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitAssignExpr(this);
            }
        }

        internal class Binary : Expr
        {
            internal Expr Left { get; }
            internal Token Operator { get; }
            internal Expr Right { get; }

            internal Binary(Expr left, Token op, Expr right)
            {
                Left = left;
                Operator = op;
                Right = right;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitBinaryExpr(this);
            }
        }

        internal class Call : Expr
        {
            internal Expr Callee { get; }
            internal Token Paren { get; }
            internal List<Expr> Arguments { get; }

            internal Call(Expr callee, Token paren, List<Expr> arguments)
            {
                Callee = callee;
                Paren = paren;
                Arguments = arguments;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitCallExpr(this);
            }
        }

        internal class Get : Expr
        {
            internal Expr Object { get; }
            internal Token Name { get; }

            internal Get(Expr obj, Token name)
            {
                Object = obj;
                Name = name;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitGetExpr(this);
            }
        }

        internal class Grouping : Expr
        {
            internal Expr Expression { get; }

            internal Grouping(Expr expr)
            {
                Expression = expr;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitGroupingExpr(this);
            }
        }

        internal class Literal : Expr
        {
            internal object Value { get; }

            internal Literal(object value)
            {
                Value = value;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitLiteralExpr(this);
            }
        }

        internal class Logical : Expr
        {
            internal Expr Left { get; }
            internal Token Operator { get; }
            internal Expr Right { get; }

            internal Logical(Expr left, Token op, Expr right)
            {
                Left = left;
                Operator = op;
                Right = right;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitLogicalExpr(this);
            }
        }

        internal class Set : Expr
        {
            internal Expr Object { get; }
            internal Token Name { get; }
            internal Expr Value { get; }

            internal Set(Expr obj, Token name, Expr value)
            {
                Object = obj;
                Name = name;
                Value = value;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitSetExpr(this);
            }
        }

        internal class Super : Expr
        {
            internal Token Keyword { get; }
            internal Token Method { get; }

            internal Super(Token keyword, Token method)
            {
                Keyword = keyword;
                Method = method;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitSuperExpr(this);
            }
        }

        internal class This : Expr
        {
            internal Token Keyword { get; }

            internal This(Token keyword)
            {
                Keyword = keyword;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitThisExpr(this);
            }
        }

        internal class Unary : Expr
        {
            internal Token Operator { get; }
            internal Expr Right { get; }

            internal Unary(Token op, Expr right)
            {
                Operator = op;
                Right = right;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitUnaryExpr(this);
            }
        }

        internal class Variable : Expr
        {
            internal Token Name { get; }

            internal Variable(Token name)
            {
                Name = name;
            }

            internal override R Accept<R>(IVisitor<R> visitor)
            {
                return visitor.VisitVariableExpr(this);
            }
        }
    }
}