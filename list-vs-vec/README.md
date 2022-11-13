# Inserting in the middle of a linked list / a vector

This micro benchmarn inserts N numbers into a list/vector, inserting each new
number into the middle. Cursor position between iterations __is not__ maintained
for a linked list.

In effect, this measures the cost of traversing n linked list nodes vs memcpying
2n vec elements.

The print outs are for cummulative times to insert 10**k elements this way.

Results:

```
1 vec  204.00ns
1 list 167.00ns
10 vec  694.00ns
10 list 620.00ns
100 vec  2.44µs
100 list 6.37µs
1000 vec  25.04µs
1000 list 331.94µs
10000 vec  1.27ms
10000 list 30.90ms
100000 vec  244.41ms
100000 list 3.16s
```
