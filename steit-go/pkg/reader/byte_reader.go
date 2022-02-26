package reader

var _ IReader = (*ByteReader)(nil)

type ByteReader struct {
	bytes  []byte
	offset int
}

func NewByteReader(bytes []byte) IReader {
	return &ByteReader{bytes: bytes, offset: 0}
}

func (b *ByteReader) Remaining() int {
	return len(b.bytes) - b.offset
}

func (b *ByteReader) ReadUint8() byte {
	if b.Remaining() <= 0 {
		panic("end of stream")
	}

	n := b.bytes[b.offset]
	b.offset++

	return n
}

func (b *ByteReader) Read(count int) []byte {
	if b.Remaining() <= 0 {
		panic("end of stream")
	}

	bytes := make([]byte, count)

	for i := 0; i < count; i++ {
		bytes[i] = b.bytes[b.offset]
		b.offset++
	}

	return bytes
}

func (b *ByteReader) Skip(count int) {
	if b.Remaining() < count {
		panic("end of stream")
	}
	b.offset += count
}
