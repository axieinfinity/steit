package state

import (
	"errors"
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
)

type deserializeOptArgs struct {
	tag *uint32
}

type DeserializeOptArgs func(*deserializeOptArgs)

func Deserialize(_ reflect.Type, r readerpkg.IReader, path *path.Path, opts ...DeserializeOptArgs) IState {
	return nil
}

func DeserializeWithTag(tag uint32) DeserializeOptArgs {
	return func(doa *deserializeOptArgs) {
		doa.tag = &tag
	}
}

type Deserializer interface {
	Deserialize(reader readerpkg.IReader, path *path.Path) error
}

func DeserializeNested(value Deserializer, r readerpkg.IReader, path *path.Path, tag uint32) error {
	if value == nil {
		return errors.New("nil value")
	}

	err := value.Deserialize(r, path)
	if err != nil {
		return err
	}
	return nil
}

func DeserializeState(value interface{}, r readerpkg.IReader, path *path.Path) IState {
	var state IState
	if state == nil {
		panic("not supported")
	}
	return state
}

func DeserializePrimitive(value interface{}, r readerpkg.IReader) error {
	switch value.(type) {
	case *uint32:
		data := readerpkg.ReadUInt32(r)
		reflect.ValueOf(value).Elem().Set(reflect.ValueOf(&data).Elem())
		return nil
	default:
		panic("not supported")
	}
}

func IsStateType(_type reflect.Type) bool {
	istate := reflect.TypeOf((*IState)(nil)).Elem()
	return _type.Implements(istate)
}

func IsPrimitiveType(_type reflect.Type) bool {
	return false
}

func Construct(reflect.Type, *path.Path) interface{} {
	return nil
}

func ValidateType(_type reflect.Type) error {
	if !IsStateType(_type) && !IsPrimitiveType(_type) {
		return errors.New("type not supported, failed validate type")
	}
	return nil
}
