package main

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func IsInvalid(x int, onlyTwo bool) bool {
	var repr = strconv.Itoa(x)
	var sl = len(repr)
	if onlyTwo {
		if sl%2 == 0 {
			if repr[:sl/2] == repr[sl/2:] {
				return true
			}
		}
	} else {
		// TODO
		return true
	}
	return false
}

func RepetitionsInRangeBruteForce(f int, t int) int {
	var total int = 0
	for x := f; x <= t; x++ {
		if IsInvalid(x, true) {
			total += x
		}
	}
	return total
}

func main() {
	b_input, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	input := strings.Trim(string(b_input), " \n")
	var intervals = strings.Split(input, ",")
	var solution1 int = 0
	for _, interval := range intervals {
		var nums = strings.Split(interval, "-")
		f, err := strconv.Atoi(nums[0])
		if err != nil {
			panic(err)
		}
		t, err := strconv.Atoi(nums[1])
		if err != nil {
			panic(err)
		}
		solution1 += RepetitionsInRangeBruteForce(f, t)
	}
	fmt.Printf("SOLUTION PART 1: %d", solution1)

}
