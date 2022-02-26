package state

import (
	"errors"
	"reflect"

	"github.com/axieinfinity/steit/steit-go/pkg/path"
	"github.com/axieinfinity/steit/steit-go/pkg/reader"
	readerpkg "github.com/axieinfinity/steit/steit-go/pkg/reader"
)

type deserializeOptArgs struct {
	tag *uint32
}

type DeserializeOptArgs func(*deserializeOptArgs)

func Deserialize(value Deserializer, r readerpkg.IReader, path *path.Path, opts ...DeserializeOptArgs) error {
	if value == nil {
		return errors.New("nil value")
	}
	args := &deserializeOptArgs{}
	for _, op := range opts {
		op(args)
	}
	var err error
	if args.tag != nil {
		if _, ok := value.(IState); ok {
			err = value.Deserialize(r, path.GetNested(*args.tag))
		} else {
			err = value.Deserialize(r, path)
		}
	} else {
		err = value.Deserialize(r, path)
	}

	return err

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

	var err error
	if _, ok := value.(IState); ok {
		err = value.Deserialize(reader.GetNested(r), path)
	} else {
		err = value.Deserialize(r, path)
	}

	return err
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
