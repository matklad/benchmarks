package main

import (
	"fmt"
	"strconv"
	"time"
)

const N = 50_000_000

func main() {
	m := make(map[string]uint64)

	t := time.Now()
	for i := uint64(0); i < N; i++ {
		m[strconv.FormatUint(i, 10)] = i
	}
	fmt.Println(time.Since(t).Milliseconds())

	t = time.Now()
	total := uint64(0)
	for i := uint64(0); i < N; i++ {
		total += m[strconv.FormatUint(i, 10)];
	}
	fmt.Println(time.Since(t).Milliseconds())
	fmt.Println(total)
}
