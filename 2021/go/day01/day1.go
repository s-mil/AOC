package main

import (
	"fmt"
	"bufio"
	"io"
	"os"
)

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func read_input(f string) []int {
    dat, err := os.ReadFile(f)
	check(err)
	fmt.Print(string(dat))

	data := make([]int)
	
}

func main() {
	fmt.Println("Hello, World!")
}
