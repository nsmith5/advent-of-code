package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
)

func isDigit(b byte) bool {
	return b >= 48 && b <= 57
}

func asDigit(b byte) int {
	return int(b - 48)
}

func firstDigitPart1(line []byte) int {
	for i := 0; i < len(line); i++ {
		if isDigit(line[i]) {
			return asDigit(line[i])
		}
	}
	// unreachable for valid input
	return -1
}

func lastDigitPart1(line []byte) int {
	for i := 0; i < len(line); i++ {
		if isDigit(line[len(line)-1-i]) {
			return asDigit(line[len(line)-1-i])
		}
	}
	// unreachable for valid input
	return -1
}

func digitFromString(s string) int {
	switch string(s) {
	case "one":
		return 1
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	}

	return -1
}

func digitFromReversedString(s string) int {
	switch string(s) {
	case "eno":
		return 1
	case "owt":
		return 2
	case "eerht":
		return 3
	case "ruof":
		return 4
	case "evif":
		return 5
	case "xis":
		return 6
	case "neves":
		return 7
	case "thgie":
		return 8
	case "enin":
		return 9
	}

	return -1
}

func firstDigitPart2(line []byte) int {
	pattern := regexp.MustCompile("(one|two|three|four|five|six|seven|eight|nine|[0-9])")

	match := pattern.Find(line)
	if match == nil {
		// unreachable for valid input
		return -1
	}

	if len(match) == 1 {
		return asDigit(match[0])
	}

	return digitFromString(string(match))
}

func lastDigitPart2(line []byte) int {
	// For silly reasons, golang regexp doesn't support overlapping matches.
	// This means that "twone", for example, will not return "one" as the final
	// digit because it overlaps with the "two" match in the same string.
	//
	// This means, we reverse the string first!

	reversed := make([]byte, len(line), len(line))
	for i := 0; i < len(line); i++ {
		reversed[i] = line[len(line)-1-i]
	}

	// Lol wat.
	pattern := regexp.MustCompile("(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])")

	match := pattern.Find(reversed)
	if match == nil {
		// unreachable for valid input
		return -1
	}

	if len(match) == 1 {
		return asDigit(match[0])
	}

	return digitFromReversedString(string(match))
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	var sum int
	b := bufio.NewScanner(f)
	for b.Scan() {
		line := b.Bytes()
		first := firstDigitPart2(line)
		second := lastDigitPart2(line)
		sum += (10 * first) + second
	}

	fmt.Println(sum)
}
