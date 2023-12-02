package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Sample struct {
	Red   int
	Blue  int
	Green int
}

type Game struct {
	ID      int
	Samples []Sample
}

func ParseGame(line string) (*Game, error) {
	var g Game
	{
		parts := strings.Split(line, ":")
		if len(parts) != 2 {
			return nil, errors.New("bad format: missing `:`")
		}
		line = parts[1]

		id, ok := strings.CutPrefix(parts[0], "Game ")
		if !ok {
			return nil, errors.New("bad format: game id")
		}
		var err error
		g.ID, err = strconv.Atoi(id)
		if err != nil {
			return nil, err
		}
	}

	// Trim leading and trailing spaces
	samples := strings.Split(line, ";")
	for _, sample := range samples {
		var s Sample
		for _, count := range strings.Split(sample, ",") {
			count = strings.Trim(count, " ")
			parts := strings.Split(count, " ")
			if len(parts) != 2 {
				return nil, errors.New("bad format: sample")
			}
			num, err := strconv.Atoi(parts[0])
			if err != nil {
				return nil, err
			}

			switch parts[1] {
			case "red":
				s.Red = num
			case "blue":
				s.Blue = num
			case "green":
				s.Green = num
			}
		}

		g.Samples = append(g.Samples, s)
	}

	return &g, nil
}

func IsPossible(g Game) bool {
	for _, sample := range g.Samples {
		if sample.Red > 12 || sample.Green > 13 || sample.Blue > 14 {
			return false
		}
	}
	return true
}

func MinColors(g Game) Sample {
	var min Sample
	for _, sample := range g.Samples {
		if sample.Red > min.Red {
			min.Red = sample.Red
		}

		if sample.Blue > min.Blue {
			min.Blue = sample.Blue
		}

		if sample.Green > min.Green {
			min.Green = sample.Green
		}
	}

	return min
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	var games []Game
	{
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			game, err := ParseGame(scanner.Text())
			if err != nil {
				log.Fatal("failed to parse game")
			}

			games = append(games, *game)
		}
	}

	var sum int
	for _, game := range games {
		if IsPossible(game) {
			sum += game.ID
		}
	}

	fmt.Println("part 1:", sum)

	var powersum int
	for _, game := range games {
		min := MinColors(game)
		powersum += min.Red * min.Blue * min.Green
	}

	fmt.Println("part 2:", powersum)
}
