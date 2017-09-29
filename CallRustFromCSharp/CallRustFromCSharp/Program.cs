using System;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

namespace CallRustFromCSharp
{
    internal class Program
    {
        [DllImport("embed.dll")]
        public static extern void process(long iterations);

        [DllImport("embed.dll")]
        public static extern long process_2(long iterations);

        public static void Main()
        {
            long iterations = 5000000000L;
            var timer = new Stopwatch();

            var tasks = Enumerable.Range(1, 10).Select(x => Task.Run(() =>
            {
                var count = 0L;

                for (long i = 0; i < iterations; i++)
                {
                    count++;
                }
            }));

            timer.Restart();
            Task.WaitAll(tasks.ToArray());
            timer.Stop();

            Console.WriteLine("C# code done by {0} ms!", timer.Elapsed.TotalMilliseconds);

            timer.Restart();
            process(iterations);
            timer.Stop();

            Console.WriteLine("Rust code done by {0} ms!", timer.Elapsed.TotalMilliseconds);

            var tasks2 = Enumerable.Range(1, 10).Select(x => Task.Run(() =>
            {
                process_2(iterations);
            }));

            timer.Restart();
            Task.WaitAll(tasks2.ToArray());
            timer.Stop();

            Console.WriteLine("Rust code with C# threads done by {0} ms!", timer.Elapsed.TotalMilliseconds);

            Console.ReadKey();
        }
    }
}
