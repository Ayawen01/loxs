using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace lox_cs
{
    internal class LexError : Exception
    {
        private char? C { get; }
        private int Line { get; }

        internal LexError(string message, int line, char? c = null) : base(message)
        {
            C = c;
            Line = line;
        }

        public override string ToString()
        {
            if (C is null)
                return string.Format("[line {0}] LexError {1}", Line, Message);
            return string.Format("[line {0}] LexError `{1}` {2}", Line, C, Message);
        }
    }

    internal class ParseError : Exception
    {
        private int Line { get; }

        internal ParseError(string message, int line) : base(message)
        {
            Line = line;
        }

        public override string ToString()
        {
            return string.Format("[line {0}] ParseError {1}", Line, Message);
        }
    }

    internal class RuntimeError : Exception
    {
        private int Line { get; }

        internal RuntimeError(string message, int line) : base(message)
        {
            Line = line;
        }

        public override string ToString()
        {
            return string.Format("[line {0}] RuntimeError {1}", Line, Message);
        }
    }
}
