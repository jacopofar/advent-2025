package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type rotation struct {
	direction    int
	value        int
	extra_rounds int
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var rotations []rotation

	for scanner.Scan() {
		line := scanner.Text()
		direction := 1
		if line[0:1] == "L" {
			direction = -1
		}
		value, _ := strconv.Atoi(line[1:])
		extra_rounds := value / 100
		value = value % 100
		rotations = append(rotations, rotation{direction, value, extra_rounds})
	}
	if err := scanner.Err(); err != nil {
		log.Println(err)
	}
	var position int = 50
	var zeroes_steps, zeroes_crossed int = 0, 0
	for step, rotation := range rotations {
		zeroes_crossed += rotation.extra_rounds
		fmt.Printf("[step %d] from %d  %d", step, position, rotation)
		var new_position int = position + rotation.direction*rotation.value
		new_position = new_position % 100
		if new_position < 0 {
			new_position = 100 + new_position
		}
		if new_position == 0 {
			zeroes_steps++
		}
		if rotation.direction == 1 && new_position < position && position != 0 && new_position != 0 {
			zeroes_crossed++
			fmt.Printf("crossed a zero")
		}
		if rotation.direction == -1 && new_position > position && position != 0 && new_position != 0 {
			zeroes_crossed++
			fmt.Printf("crossed a zero")
		}
		fmt.Printf("[step %d] %d -> %d", step, position, new_position)
		position = new_position
	}
	fmt.Printf("SOLUTION PART 1: %d", zeroes_steps)
	fmt.Printf("SOLUTION PART 2: %d", zeroes_steps+zeroes_crossed)

}
