package main

import (
    "fmt"
    "sort"
    "strconv"
    "time"
)

const N uint64 = 50000000

func main() {
    var v []string

    t := time.Now()
    for i := uint64(0); i < N; i++ {
        v = append(v, strconv.FormatUint(i, 10))
    }
    fmt.Println(time.Since(t).Milliseconds())

    t = time.Now()
    sort.Strings(v)
    fmt.Println(time.Since(t).Milliseconds())

    t = time.Now()
    var total uint64 = 0
    for i := uint64(0); i < N; i++ {
		key := strconv.FormatUint(i, 10)
		index := sort.SearchStrings(v, key)
        if index < len(v) && v[index] == key {
            total++
        }
    }
    fmt.Println(time.Since(t).Milliseconds())

    fmt.Println(total)
}
