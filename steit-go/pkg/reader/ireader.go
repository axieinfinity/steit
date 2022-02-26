package reader

type IReader interface {
	Remaining() int
	ReadUint8() byte
	Read(int) []byte
	Skip(int)
}
