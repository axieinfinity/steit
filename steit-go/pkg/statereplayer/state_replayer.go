package statereplayer

import (
	"reflect"

	"github.com/axieinfinity/steit/steit-go/pkg/logentry"
	"github.com/axieinfinity/steit/steit-go/pkg/reader"
	readerpkg "github.com/axieinfinity/steit/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit/steit-go/pkg/state"
)

func Replay(_type reflect.Type, root statepkg.IState, r readerpkg.IReader) {
	for !readerpkg.EndOfStream(r) {
		var entry = logentry.Deserialize(readerpkg.GetNested(r), nil)
		ReplayByLogEntry(_type, root, entry)
	}
}

func ReplayByLogEntry(_type reflect.Type, root statepkg.IState, entry *logentry.LogEntry) {
	path := getPath(entry)
	tag := uint32(0)

	if entry.GetTag() == logentry.UpdateTag {
		if len(path) > 0 {
			tag = path[len(path)-1]
			path = path[:len(path)-1]
		} else {
			r := reader.NewByteReader(entry.GetUpdateVariant().GetValue())
			root = statepkg.Deserialize(_type, r, root.GetPath())
			return
		}
	}

	container := statepkg.GetNested(root, path)

	if container == nil {
		return
	}

	switch entry.GetTag() {
	case logentry.UpdateTag:
		wireType := container.GetWireType(tag)
		if wireType == nil {
			return
		}
		r := reader.NewByteReader(entry.GetUpdateVariant().GetValue())
		container.ReplaceAt(tag, *wireType, r, true)
	case logentry.ListPushTag:
		byteReader := reader.NewByteReader(entry.GetListPushVariant().GetItem())
		container.ReplayListPush(byteReader)
	case logentry.ListPopTag:
		container.ReplayListPop()
	case logentry.MapRemoveTag:
		key := uint32(0)
		container.ReplayMapRemove(key)
	}
}

func getPath(entry *logentry.LogEntry) []uint32 {
	switch entry.GetTag() {
	case logentry.UpdateTag:
		panic("not implemented")
	case logentry.ListPushTag:
		panic("not implemented")
	case logentry.ListPopTag:
		panic("not implemented")
	case logentry.MapRemoveTag:
		panic("not implemented")
	}
	return nil
}
