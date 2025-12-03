package main

import (
	"bufio"
	"fmt"
	"log"
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

func LineJoltage(ratings []int) int {
	// find max on everything but last element
	fi, fm := FindIdxAndMax(ratings[:len(ratings)-1])
	// find max after that
	_, sm := FindIdxAndMax(ratings[fi+1:])
	return fm*10 + sm
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
	solution1 := 0
	for _, arr := range ratings {
		solution1 += LineJoltage(arr)
	}
	// fmt.Println(ratings)
	fmt.Printf("SOLUTION PART 1: %d", solution1)
	fmt.Printf("SOLUTION PART 2: %d", 0)
}
