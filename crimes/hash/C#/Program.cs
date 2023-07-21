using System;
using System.Collections.Generic;
using System.Diagnostics;

const ulong N = 50_000_000;

Dictionary<string, ulong> m = new Dictionary<string, ulong>();
Stopwatch t = Stopwatch.StartNew();

for (ulong i = 0; i < N; i++)
{
    m[i.ToString()] = i;
}

Console.WriteLine(t.ElapsedMilliseconds);
t.Restart();

ulong total = 0;
for (ulong i = 0; i < N; i++)
{
    total += m[i.ToString()];
}

Console.WriteLine(t.ElapsedMilliseconds);
Console.WriteLine(total);
