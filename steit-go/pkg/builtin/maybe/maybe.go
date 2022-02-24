package maybe

// const (
// 	NoneTag = 0
// 	SomeTag = 1
// )

// var _ state.IEnumState = (*Maybe)(nil)

// type Maybe struct {
// 	path        *path.Path
// 	tag         uint32
// 	variant     state.IState
// 	noneVariant *None
// 	someVariant *Some
// }

// type None struct {
// }

// type Some struct {
// }

// func (mb *Maybe) GetNoneVariant() *None {
// 	return (*None)(mb.variant)
// }

// func (mb *Maybe) GetPath() *path.Path {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) GetWireType(_ uint32) *codec.WireType {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) GetNested(_ uint32) state.IState {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) ReplayListPush(reader reader.IReader) {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) ReplayListPop() {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) ReplayMapRemove(_ uint32) {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) GetTag() uint32 {
// 	panic("not implemented") // TODO: Implement
// }

// func (mb *Maybe) GetVariant() state.IState {
// 	panic("not implemented") // TODO: Implement
// }
