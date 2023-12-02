package main

import (
	"fmt"
	"testing"
)

func TestIsDigit(t *testing.T) {
	tests := []struct {
		given byte
		want  bool
	}{
		{given: '0', want: true},
		{given: '9', want: true},
		{given: 'a', want: false},
		{given: '0' - 1, want: false},
		{given: '9' + 1, want: false},
	}

	for _, test := range tests {
		t.Run(fmt.Sprint(test.given), func(t *testing.T) {
			got := isDigit(test.given)
			if got != test.want {
				t.Error("got != want")
			}
		})
	}
}

func TestAsDigit(t *testing.T) {
	tests := []struct {
		given byte
		want  int
	}{
		{given: '0', want: 0},
		{given: '9', want: 9},
	}

	for _, test := range tests {
		t.Run(fmt.Sprint(test.given), func(t *testing.T) {
			got := asDigit(test.given)
			if got != test.want {
				t.Error("got != want")
			}
		})
	}
}

func TestFirstDigitPart1(t *testing.T) {
	tests := []struct {
		given string
		want  int
	}{
		{given: "abcd1234", want: 1},
		{given: "0", want: 0},
		{given: "abbddaa7", want: 7},
	}

	for _, test := range tests {
		t.Run(fmt.Sprint(test.given), func(t *testing.T) {
			got := firstDigitPart1([]byte(test.given))
			if got != test.want {
				t.Error("got != want")
			}
		})
	}
}

func TestLastDigitPart1(t *testing.T) {
	tests := []struct {
		given string
		want  int
	}{
		{given: "abcd1234", want: 4},
		{given: "0", want: 0},
		{given: "abbddaa7", want: 7},
	}

	for _, test := range tests {
		t.Run(fmt.Sprint(test.given), func(t *testing.T) {
			got := lastDigitPart1([]byte(test.given))
			if got != test.want {
				t.Error("got != want")
			}
		})
	}
}

func TestFirstDigitPart2(t *testing.T) {
	tests := []struct {
		given string
		want  int
	}{
		{given: "abcd1234", want: 1},
		{given: "0", want: 0},
		{given: "abbddaa7", want: 7},
		{given: "ddddoneseven", want: 1},
		// "zero" isn't an allowed digit
		{given: "zero7five", want: 7},
		{given: "twone", want: 2},
	}

	for _, test := range tests {
		t.Run(fmt.Sprint(test.given), func(t *testing.T) {
			got := firstDigitPart2([]byte(test.given))
			if got != test.want {
				t.Error("got != want")
			}
		})
	}
}

func TestLastDigitPart2(t *testing.T) {
	tests := []struct {
		given string
		want  int
	}{
		{given: "abcd1234", want: 4},
		{given: "0", want: 0},
		{given: "abbddaa7", want: 7},
		{given: "ddddoneseven", want: 7},
		{given: "zero7five", want: 5},
		{given: "twone", want: 1},
	}

	for _, test := range tests {
		t.Run(fmt.Sprint(test.given), func(t *testing.T) {
			got := lastDigitPart2([]byte(test.given))
			if got != test.want {
				t.Errorf("got = %d,  want = %d", got, test.want)
			}
		})
	}
}
