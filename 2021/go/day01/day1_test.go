package main

import (
	"regexp"
	"testing"
)

// TestHelloName calls greetings.Hello with a name, checking
// for a valid return value.
func TestPart1(t *testing.T) {
	// setup
	items := [...]int{199,200,208,210,200,207,240,269,260,263}

	// Test
	want := 7
	msg, err := Hello("Gladys")

	// Assert
	if !want.MatchString(msg) || err != nil {
		t.Fatalf(`Hello("Gladys") = %q, %v, want match for %#q, nil`, msg, err, want)
	}
}
