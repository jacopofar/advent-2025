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

	for splitLengthCandidate := sl / 2; splitLengthCandidate >= 1; splitLengthCandidate-- {
		// non-exact split
		if sl%splitLengthCandidate != 0 {
			continue
		}
		if repr == strings.Repeat(repr[:splitLengthCandidate], sl/splitLengthCandidate) {
			return true
		}
		if onlyTwo {
			// stop iterating, they asked for only a split into two and this is not it
			return false
		}
	}

	return false
}

func RepetitionsInRangeBruteForce(f int, t int, onlyTwo bool) int {
	var total int = 0
	for x := f; x <= t; x++ {
		if IsInvalid(x, onlyTwo) {
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
	var solution1, solution2 int = 0, 0
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
		solution1 += RepetitionsInRangeBruteForce(f, t, true)
		solution2 += RepetitionsInRangeBruteForce(f, t, false)

	}
	fmt.Printf("SOLUTION PART 1: %d", solution1)
	fmt.Printf("SOLUTION PART 2: %d", solution2)
}
