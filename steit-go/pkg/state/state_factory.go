package state

import (
	"errors"
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
)

type deserializeOptArgs struct {
	tag *uint32
}

type DeserializeOptArgs func(*deserializeOptArgs)

func Deserialize(_ reflect.Type, r reader.IReader, path *path.Path, opts ...DeserializeOptArgs) IState {
	return nil
}

func DeserializeWithTag(tag uint32) DeserializeOptArgs {
	return func(doa *deserializeOptArgs) {
		doa.tag = &tag
	}
}

func DeserializeNested(_ reflect.Type, r reader.IReader, path *path.Path, tag uint32) interface{} {
	return nil
}

func IsStateType(_type reflect.Type) bool {
	return false
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
