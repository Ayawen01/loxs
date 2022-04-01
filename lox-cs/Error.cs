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
            return string.Format("[line {0}] LexError `{1}` {2}", Line, C, Message);
        }
    }
}
