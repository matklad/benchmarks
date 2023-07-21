using System;
using System.Collections.Generic;
using System.Diagnostics;

const ulong N = 50_000_000;

List<string> v = new List<string>();

Stopwatch sw = Stopwatch.StartNew();
for (ulong i = 0; i < N; i++)
{
    v.Add(i.ToString());
}
Console.WriteLine(sw.ElapsedMilliseconds);

sw.Restart();
v.Sort(StringComparer.Ordinal);
Console.WriteLine(sw.ElapsedMilliseconds);

sw.Restart();
ulong total = 0;
for (ulong i = 0; i < N; i++)
{
    total += (ulong)(v.BinarySearch(i.ToString(), StringComparer.Ordinal) >= 0 ? 1 : 0);
}
Console.WriteLine(sw.ElapsedMilliseconds);

Console.WriteLine(total);
