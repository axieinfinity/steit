package codec

import "fmt"

type WireType = int

const (
	WireTypeSized  WireType = 2
	WireTypeVarint WireType = 0
)

func GetWireTypePtr(wireType WireType) *WireType {
	return &wireType
}

func NewWireType(value uint32) WireType {
	switch value {
	case uint32(WireTypeVarint):
		return WireType(value)
	case uint32(WireTypeSized):
		return WireType(value)
	default:
		panic(fmt.Sprintf("Invalid wire type value: %d", value))
	}
}
