package state

import (
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
)

type deserializeOptArgs struct {
	tag *uint32
}

type DeserializeOptArgs func(*deserializeOptArgs)

func Deserialize(r reader.IReader, path *path.Path, opts ...DeserializeOptArgs) IState {
	return nil
}

func DeserializeWithTag(tag uint32) DeserializeOptArgs {
	return func(doa *deserializeOptArgs) {
		doa.tag = &tag
	}
}

func DeserializeNested(r reader.IReader, path *path.Path, tag uint32) interface{} {
	return nil
}

func IsStateType(_type reflect.Type) bool {
	return false
}
