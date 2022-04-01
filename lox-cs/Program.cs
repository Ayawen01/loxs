namespace lox_cs
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length > 1)
            {
                Console.WriteLine("Usage: lox [script]");
            }
            else if (args.Length == 1)
            {
                RunFile(args[0]);
            }
            else
            {
                RunPrompt();
            }
        }

        private static void Run(string? source)
        {
            if (source is null)
            {
                throw new Exception("source code is null.");
            }

            try
            {
                var scanner = new Scanner(source);
                scanner.ScanTokens();
                scanner.Tokens.ForEach(t => Console.WriteLine(t));
            }
            catch (LexError e)
            {
                Console.WriteLine(e.ToString());
            }
        }

        private static void RunFile(string path)
        {
            string source = File.ReadAllText(path);
            Run(source);
        }

        private static void RunPrompt()
        {
            while (true)
            {
                Console.Write("> ");
                string? input = Console.ReadLine();
                Run(input);
            }
        }
    }
}