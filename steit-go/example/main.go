package main

import (
	"encoding/json"
	"fmt"

	"github.com/axieinfinity/steit/steit-go/origin"
	readerpkg "github.com/axieinfinity/steit/steit-go/pkg/reader"
)

func main() {
	decodeBinaryMessage()
	decodeHelloSample()
	decodeCardPlayHint()
	decodeActionMessage()
}

func decodeBinaryMessage() {
	bMsg := origin.BinaryMessage{}

	data := []byte{0, 2, 155, 10, 10, 226, 9, 52, 25, 18, 46, 45, 0, 2, 42, 33, 8, 2, 1, 6, 10, 27, 2, 8, 10, 1, 110, 18, 1, 0, 24, 1, 10, 1, 0, 26, 1, 0, 34, 1, 0, 42, 1, 0, 50, 1, 0, 72, 14, 7, 0, 2, 1, 8, 10, 1, 1, 26, 1, 0, 33, 2, 10, 12, 11, 0, 2, 8, 7, 0, 2, 1, 1, 10, 1, 1, 26, 14, 13, 0, 2, 10, 9, 0, 2, 3, 3, 0, 2, 10, 1, 3, 32, 3, 189, 1, 7, 42, 182, 1, 180, 1, 5, 2, 3, 2, 1, 0, 10, 3, 2, 1, 3, 16, 10, 26, 164, 1, 162, 1, 0, 2, 158, 1, 6, 9, 2, 3, 4, 0, 1, 65, 0, 2, 4, 4, 0, 1, 10, 10, 56, 2, 40, 0, 17, 8, 8, 18, 12, 0, 2, 7, 0, 2, 4, 5, 2, 1, 3, 8, 1, 26, 4, 0, 2, 1, 0, 34, 1, 0, 50, 1, 2, 66, 8, 1, 0, 1, 4, 1, 4, 1, 15, 18, 3, 1, 0, 2, 26, 3, 0, 0, 1, 64, 65, 72, 1, 64, 8, 2, 5, 4, 0, 0, 1, 0, 10, 54, 2, 38, 0, 10, 8, 8, 18, 12, 0, 2, 7, 5, 2, 4, 4, 2, 1, 3, 8, 1, 26, 4, 0, 2, 1, 0, 34, 1, 0, 50, 1, 1, 66, 6, 1, 0, 1, 4, 1, 4, 18, 3, 1, 0, 1, 26, 3, 0, 0, 1, 64, 70, 72, 1, 9, 0, 2, 3, 3, 0, 3, 10, 1, 17, 9, 0, 2, 3, 3, 0, 4, 10, 1, 1, 50, 1, 10, 188, 1, 7, 42, 181, 1, 179, 1, 5, 2, 3, 2, 1, 0, 10, 3, 2, 1, 3, 16, 5, 26, 163, 1, 161, 1, 0, 2, 157, 1, 6, 9, 2, 3, 4, 0, 1, 66, 0, 2, 4, 4, 0, 1, 5, 10, 57, 2, 40, 0, 16, 8, 8, 18, 12, 0, 2, 7, 3, 2, 4, 4, 2, 1, 3, 8, 1, 26, 4, 0, 2, 1, 0, 34, 1, 0, 50, 1, 2, 66, 8, 1, 0, 1, 4, 1, 4, 1, 21, 18, 3, 1, 0, 2, 26, 3, 0, 0, 2, 64, 145, 1, 72, 1, 62, 8, 2, 5, 4, 0, 0, 0, 0, 10, 52, 2, 38, 0, 5, 8, 8, 18, 12, 0, 2, 7, 2, 2, 4, 5, 2, 1, 8, 8, 1, 26, 4, 0, 2, 1, 1, 34, 1, 0, 50, 1, 0, 66, 6, 1, 0, 1, 4, 1, 4, 18, 1, 1, 26, 3, 0, 0, 1, 64, 70, 72, 1, 9, 0, 2, 3, 3, 0, 3, 10, 1, 16, 9, 0, 2, 3, 3, 0, 4, 10, 1, 2, 50, 1, 5, 183, 1, 7, 42, 176, 1, 174, 1, 5, 2, 3, 2, 1, 0, 10, 3, 2, 1, 3, 16, 12, 26, 158, 1, 156, 1, 0, 2, 152, 1, 6, 9, 2, 3, 4, 0, 1, 65, 0, 2, 4, 4, 0, 1, 12, 10, 56, 2, 40, 0, 15, 8, 8, 18, 12, 0, 2, 7, 3, 2, 4, 3, 2, 1, 1, 8, 1, 26, 4, 0, 2, 1, 1, 34, 1, 0, 50, 1, 2, 66, 8, 1, 0, 1, 4, 1, 4, 1, 21, 18, 3, 1, 0, 2, 26, 3, 0, 0, 1, 64, 60, 72, 1, 58, 8, 2, 5, 4, 0, 0, 2, 0, 10, 48, 2, 36, 0, 12, 8, 8, 18, 12, 0, 2, 7, 0, 2, 4, 0, 2, 1, 3, 8, 1, 26, 4, 1, 2, 1, 0, 34, 1, 0, 50, 1, 2, 66, 4, 1, 1, 1, 15, 18, 3, 1, 0, 2, 26, 3, 0, 0, 1, 9, 0, 2, 3, 3, 0, 3, 10, 1, 15, 9, 0, 2, 3, 3, 0, 4, 10, 1, 3, 50, 1, 12, 197, 1, 7, 42, 190, 1, 188, 1, 5, 2, 3, 2, 1, 0, 10, 3, 2, 1, 3, 16, 7, 26, 172, 1, 170, 1, 0, 2, 166, 1, 6, 9, 2, 3, 4, 0, 1, 71, 0, 2, 4, 4, 0, 1, 7, 10, 62, 2, 42, 0, 14, 8, 8, 18, 12, 0, 2, 7, 0, 2, 4, 2, 2, 1, 1, 8, 1, 26, 4, 0, 2, 1, 0, 34, 1, 0, 50, 1, 2, 66, 10, 1, 0, 1, 4, 1, 4, 1, 15, 1, 25, 18, 3, 1, 0, 2, 26, 3, 0, 0, 1, 40, 1, 64, 65, 72, 1, 96, 1, 66, 8, 2, 5, 4, 0, 0, 1, 0, 10, 56, 2, 38, 0, 7, 8, 8, 18, 12, 0, 2, 7, 4, 2, 4, 1, 2, 1, 1, 8, 1, 26, 4, 0, 2, 1, 1, 34, 1, 0, 50, 1, 1, 66, 6, 1, 0, 1, 4, 1, 4, 18, 3, 1, 0, 1, 26, 3, 0, 0, 1, 48, 1, 64, 80, 72, 1, 9, 0, 2, 3, 3, 0, 3, 10, 1, 14, 9, 0, 2, 3, 3, 0, 4, 10, 1, 4, 50, 1, 7, 191, 1, 7, 42, 184, 1, 182, 1, 5, 2, 3, 2, 1, 0, 10, 3, 2, 1, 3, 16, 17, 26, 166, 1, 164, 1, 0, 2, 160, 1, 6, 9, 2, 3, 4, 0, 1, 65, 0, 2, 4, 4, 0, 1, 10, 10, 56, 2, 40, 0, 13, 8, 8, 18, 12, 0, 2, 7, 0, 2, 4, 1, 2, 1, 3, 8, 1, 26, 4, 0, 2, 1, 0, 34, 1, 0, 50, 1, 2, 66, 8, 1, 0, 1, 4, 1, 4, 1, 21, 18, 3, 1, 0, 2, 26, 3, 0, 0, 1, 64, 70, 72, 1, 66, 8, 2, 5, 4, 0, 0, 2, 0, 10, 56, 2, 40, 0, 17, 8, 8, 18, 12, 0, 2, 7, 0, 2, 4, 5, 2, 1, 3, 8, 1, 26, 4, 0, 2, 1, 0, 34, 1, 0, 50, 1, 2, 66, 8, 1, 0, 1, 4, 1, 4, 1, 15, 18, 3, 1, 0, 2, 26, 3, 0, 0, 1, 64, 65, 72, 1, 9, 0, 2, 3, 3, 0, 3, 10, 1, 13, 9, 0, 2, 3, 3, 0, 4, 10, 1, 5, 50, 1, 17, 119, 32, 2, 116, 115, 0, 2, 112, 9, 0, 2, 3, 13, 0, 9, 10, 1, 1, 9, 0, 2, 3, 13, 4, 9, 10, 1, 0, 11, 0, 2, 3, 13, 4, 5, 10, 3, 0, 0, 4, 9, 0, 2, 3, 13, 3, 9, 10, 1, 1, 9, 0, 2, 3, 13, 1, 9, 10, 1, 1, 9, 0, 2, 3, 13, 2, 9, 10, 1, 1, 12, 0, 2, 3, 5, 3, 12, 10, 4, 1, 1, 1, 1, 10, 0, 2, 3, 5, 4, 12, 10, 2, 1, 2, 12, 0, 2, 3, 5, 5, 12, 10, 4, 1, 1, 1, 1, 12, 0, 2, 3, 5, 6, 12, 10, 4, 1, 3, 1, 4, 84, 19, 0, 1, 34, 68, 67, 0, 2, 64, 45, 8, 2, 3, 4, 1, 2, 10, 37, 2, 21, 0, 19, 8, 8, 18, 4, 1, 2, 1, 12, 26, 4, 1, 2, 1, 0, 34, 1, 0, 40, 1, 8, 1, 18, 1, 0, 26, 1, 0, 32, 1, 40, 1, 72, 1, 9, 0, 2, 3, 3, 1, 4, 10, 1, 1, 7, 0, 2, 1, 7, 10, 1, 19, 42, 1, 19, 50, 6, 5, 2, 1, 3, 8, 1, 18, 44, 11, 0, 5, 8, 1, 18, 2, 3, 4, 34, 1, 1, 7, 0, 10, 8, 1, 18, 1, 3, 7, 0, 7, 8, 1, 18, 1, 3, 7, 0, 12, 8, 1, 18, 1, 2, 7, 0, 17, 8, 1, 18, 1, 3, 24, 232, 196, 196, 161, 12, 32, 90}

	reader := readerpkg.NewByteReader(data)
	bMsg.Deserialize(reader, nil)
	fmt.Println(bMsg.Variant.(*origin.BinaryMessageActionMessage).Message.CardPlayHints.GetItems()[0])
	fmt.Println(bMsg.GetNested(0).GetNested(0).GetNested(2))
	// fmt.Printf("%+v\n", bMsg.ActionMessageVariant())

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

func decodeCardPlayHint() {
	data := []byte{
		0, 1, 8, 1, 18, 3, 1, 2, 3, 26, 1, 1, 34, 1, 1, 42, 3, 1, 2, 3, 58, 1, 1, 66, 3, 1, 2, 3,
	}
	fmt.Println(len(data))
	bMsg := origin.CardPlayHint{}
	reader := readerpkg.NewByteReader(data)
	bMsg.Deserialize(reader, nil)

	_json, _ := json.Marshal(bMsg)
	fmt.Println(string(_json))
}

func decodeActionMessage() {
	data := []byte{
		0, 1, 18, 87, 28, 0, 1, 8, 1, 18, 3, 1, 2, 3, 26, 1, 1, 34, 1, 1, 42, 3, 1, 2, 3, 58, 1, 1, 66, 3, 1, 2, 3, 28, 0, 1, 8, 1, 18, 3, 1, 2, 3, 26, 1, 1, 34, 1, 1, 42, 3, 1, 2, 3, 58, 1, 1, 66, 3, 1, 2, 3, 28, 0, 1, 8, 1, 18, 3, 1, 2, 3, 26, 1, 1, 34, 1, 1, 42, 3, 1, 2, 3, 58, 1, 1, 66, 3, 1, 2, 3, 24, 142, 171, 204, 10, 32, 20,
	}

	bMsg := origin.ActionMessage{}
	reader := readerpkg.NewByteReader(data)
	bMsg.Deserialize(reader, nil)

	_json, _ := json.Marshal(bMsg)
	fmt.Println(bMsg.GetNested(2))
	fmt.Println(string(_json))
}
