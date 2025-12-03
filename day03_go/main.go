package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
)

func FindIdxAndMax(arr []int) (int, int) {
	curMax := arr[0]
	curIdx := 0
	for i := 1; i < len(arr); i++ {
		if arr[i] > curMax {
			curMax = arr[i]
			curIdx = i
		}
	}
	return curIdx, curMax
}

func LineJoltage(ratings []int, numdigits int) int {
	fmt.Println("line", ratings)
	found_digits := []int{}
	// find max on everything but last needed elements
	di, dm := FindIdxAndMax(ratings[:len(ratings)-(numdigits-1)])
	found_digits = append(found_digits, dm)
	for needed_digits := numdigits - 1; needed_digits > 0; needed_digits-- {
		// find max after that
		di_new, dm := FindIdxAndMax(ratings[di+1 : len(ratings)-(needed_digits-1)])
		// di_new is relative to the slice
		di = di_new + di + 1

		found_digits = append(found_digits, dm)
	}
	fmt.Println("result", found_digits)
	retval := 0
	for p := range numdigits {
		// fmt.Println("exp", int(math.Pow(10, float64(p))))
		retval += int(math.Pow(10, float64(p))) * found_digits[len(found_digits)-p-1]
	}
	return retval
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var ratings [][]int

	for scanner.Scan() {
		line := scanner.Text()
		line_ratings := []int{}

		for _, r := range line {
			line_ratings = append(line_ratings, (int(r) - '0'))
		}
		ratings = append(ratings, line_ratings)
	}
	if err := scanner.Err(); err != nil {
		log.Println(err)
	}
	solution1, solution2 := 0, 0
	for _, arr := range ratings {
		solution1 += LineJoltage(arr, 2)
		solution2 += LineJoltage(arr, 12)

	}
	// fmt.Println(ratings)
	fmt.Printf("SOLUTION PART 1: %d", solution1)
	fmt.Printf("SOLUTION PART 2: %d", solution2)
}
