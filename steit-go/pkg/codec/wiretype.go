package codec

type WireType = int

const (
	WireTypeSized  WireType = 2
	WireTypeVarint WireType = 0
)
