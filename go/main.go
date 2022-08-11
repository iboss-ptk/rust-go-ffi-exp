package main

import "C"

//export Hello
func Hello(n string) *C.char {
	return C.CString("Hello, " + n)
}

func main() {}
