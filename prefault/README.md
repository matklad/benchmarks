Check the effect of MAP_POPULATE:

```console
$ ./zig/zig version
0.11.0

$ ./zig/zig build-exe -Doptimize=ReleaseFast main.zig

$ perf stat -- ./main --no-populate
info: allocating 40960MiB
info: alloc: 0μs
info: touch: 9470μs
info: read:  153μs sideffect=20
info: free:  1021μs

 Performance counter stats for './main --no-populate':

         10,645.62 msec task-clock                       #    1.000 CPUs utilized
                27      context-switches                 #    2.536 /sec
                 4      cpu-migrations                   #    0.376 /sec
        10,485,777      page-faults                      #  984.985 K/sec
    48,678,155,032      cpu_core/cycles/                 #    4.573 G/sec
     <not counted>      cpu_atom/cycles/                                              (0.00%)
    65,212,987,501      cpu_core/instructions/           #    6.126 G/sec
     <not counted>      cpu_atom/instructions/                                        (0.00%)
    13,670,284,878      cpu_core/branches/               #    1.284 G/sec
     <not counted>      cpu_atom/branches/                                            (0.00%)
         7,183,892      cpu_core/branch-misses/          #  674.821 K/sec
     <not counted>      cpu_atom/branch-misses/                                       (0.00%)
   292,041,808,548      cpu_core/slots/                  #   27.433 G/sec
    76,732,553,618      cpu_core/topdown-retiring/       #     26.1% Retiring
    10,307,357,948      cpu_core/topdown-bad-spec/       #      3.5% Bad Speculation
    55,382,530,452      cpu_core/topdown-fe-bound/       #     18.9% Frontend Bound
   151,174,583,248      cpu_core/topdown-be-bound/       #     51.5% Backend Bound
    16,033,667,920      cpu_core/topdown-heavy-ops/      #      5.5% Heavy Operations       #     20.7% Light Operations
     2,290,523,988      cpu_core/topdown-br-mispredict/  #      0.8% Branch Mispredict      #      2.7% Machine Clears
    26,366,788,575      cpu_core/topdown-fetch-lat/      #      9.0% Fetch Latency          #      9.9% Fetch Bandwidth
   105,364,103,476      cpu_core/topdown-mem-bound/      #     35.9% Memory Bound           #     15.6% Core Bound

      10.646402691 seconds time elapsed

       0.845013000 seconds user
       9.801153000 seconds sys

$ perf stat -- ./main --populate
info: allocating 40960MiB
info: alloc: 7830μs
info: touch: 218μs
info: read:  194μs sideffect=20
info: free:  1120μs

 Performance counter stats for './main --populate':

          9,364.31 msec task-clock                       #    1.000 CPUs utilized
                31      context-switches                 #    3.310 /sec
                 7      cpu-migrations                   #    0.748 /sec
                17      page-faults                      #    1.815 /sec
    42,913,327,183      cpu_core/cycles/                 #    4.583 G/sec
     <not counted>      cpu_atom/cycles/                                              (0.00%)
    64,254,810,045      cpu_core/instructions/           #    6.862 G/sec
     <not counted>      cpu_atom/instructions/                                        (0.00%)
    13,185,180,235      cpu_core/branches/               #    1.408 G/sec
     <not counted>      cpu_atom/branches/                                            (0.00%)
         7,199,725      cpu_core/branch-misses/          #  768.847 K/sec
     <not counted>      cpu_atom/branch-misses/                                       (0.00%)
   256,672,306,464      cpu_core/slots/                  #   27.410 G/sec
    65,426,274,196      cpu_core/topdown-retiring/       #     25.6% Retiring
     1,006,558,064      cpu_core/topdown-bad-spec/       #      0.4% Bad Speculation
    29,190,183,872      cpu_core/topdown-fe-bound/       #     11.4% Frontend Bound
   160,042,732,265      cpu_core/topdown-be-bound/       #     62.6% Backend Bound
     8,052,464,516      cpu_core/topdown-heavy-ops/      #      3.1% Heavy Operations       #     22.4% Light Operations
     1,006,558,064      cpu_core/topdown-br-mispredict/  #      0.4% Branch Mispredict      #      0.0% Machine Clears
    11,278,064,281      cpu_core/topdown-fetch-lat/      #      4.4% Fetch Latency          #      7.0% Fetch Bandwidth
   145,950,919,361      cpu_core/topdown-mem-bound/      #     57.1% Memory Bound           #      5.5% Core Bound

       9.365654834 seconds time elapsed

       0.412920000 seconds user
       8.952281000 seconds sys
```

Important note: the benchmark bypasses allocator interface. Turns out that Zig memsets allocated
memory even in ReleaseFast mode!
