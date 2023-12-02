package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestParseGame(t *testing.T) {
	tests := map[string]struct {
		given  string
		expect Game
	}{
		"simple": {
			given: "Game 33: 1 red, 2 blue, 3 green",
			expect: Game{
				ID: 33,
				Samples: []Sample{
					{Red: 1, Blue: 2, Green: 3},
				},
			},
		},
		"two samples": {
			given: "Game 4123: 100 red, 23 blue, 13 green; 7 red, 13 blue, 17 green",
			expect: Game{
				ID: 4123,
				Samples: []Sample{
					{Red: 100, Blue: 23, Green: 13},
					{Red: 7, Blue: 13, Green: 17},
				},
			},
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			got, err := ParseGame(test.given)
			if err != nil {
				t.Fatal(err)
			}

			if diff := cmp.Diff(*got, test.expect); diff != "" {
				t.Error(diff)
			}
		})
	}
}
