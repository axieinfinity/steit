package reader

import (
	"fmt"

	"github.com/axieinfinity/steit/steit-go/pkg/codec"
)

const (
	WireTypeBits uint32 = 3
	WireTypeMask uint32 = (uint32(1) << WireTypeBits) - 1
)

type Reader struct {
}

func EndOfStream(reader IReader) bool {
	return reader.Remaining() <= 0
}

func ReadToEnd(reader IReader) []byte {
	return reader.Read(reader.Remaining())
}

func ReadUnsignedVarint(reader IReader) uint64 {
	var value uint64
	offset := 0
	var octet byte

	for {
		octet = reader.ReadUint8()
		value |= uint64((octet & 0x7f)) << offset

		if (octet & 0x80) == 0 {
			return value
		}

		offset += 7
	}
}

func ReadSignedVarint(reader IReader) int64 {
	var value int64
	value = int64(ReadUnsignedVarint(reader))
	return (value >> 1) ^ -(value & 1)
}

func ReadByte(reader IReader) byte {
	return byte(ReadUnsignedVarint(reader))
}

func ReadUInt16(reader IReader) uint16 {
	return uint16(ReadUnsignedVarint(reader))
}

func ReadUInt32(reader IReader) uint32 {
	return uint32(ReadUnsignedVarint(reader))
}

func ReadUInt64(reader IReader) uint64 {
	return ReadUnsignedVarint(reader)
}

func ReadInt16(reader IReader) int16 {
	return int16(ReadSignedVarint(reader))
}

func ReadInt32(reader IReader) int32 {
	return int32(ReadSignedVarint(reader))
}

func ReadInt64(reader IReader) int64 {
	return ReadSignedVarint(reader)
}

func ReadBoolean(reader IReader) bool {
	var value bool
	value = false
	var octet byte

	for {
		octet = reader.ReadUint8()
		value = value || ((octet & 0x7f) != 0)

		if (octet & 0x80) == 0 {
			return value
		}
	}
}

func ReadString(reader IReader) string {
	panic("unimplemented")
}

// Tag(29 bits) | WireType (3 bits)
func ReadKey(reader IReader) (uint32, codec.WireType) {
	key := ReadUInt32(reader)
	tag := key >> WireTypeBits
	wireType := codec.NewWireType(key & WireTypeMask)

	return tag, wireType
}

func ReadSize(reader IReader) int {
	return int(ReadUInt32(reader))
}

func SkipToEnd(reader IReader) {
	reader.Skip(reader.Remaining())
}

func SkipField(reader IReader, wireType codec.WireType) {
	switch wireType {
	case codec.WireTypeVarint:
		ReadBoolean(reader)
	case codec.WireTypeSized:
		SkipToEnd(reader)
	default:
		panic(fmt.Sprintf("Unsupported wire type: %d", wireType))
	}
}

func GetNested(reader IReader) IReader {
	bytes := reader.Read(ReadSize(reader))
	return NewByteReader(bytes)
}
