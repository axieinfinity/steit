package main

import (
	"fmt"

	readerpkg "github.com/axieinfinity/steit/steit-go/pkg/reader"
)

func main() {
	decodeHelloSample()
}

func decodeHelloSample() {
	data := []byte{
		// Numbers: 1, 2, 1337.
		2, 4, 2, 4, 242, 20,
		// Others: -1, -2, 1337.
		10, 4, 1, 3, 242, 20,
	}
	reader := readerpkg.NewByteReader(data)
	helloMsg := Hello{}
	helloMsg.Deserialize(reader, nil)
	fmt.Println(helloMsg.GetNested(0))
	fmt.Println(helloMsg.GetNested(1))
}
