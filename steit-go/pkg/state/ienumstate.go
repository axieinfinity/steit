package state

type IEnumState interface {
	IState
	GetTag() uint32
	GetVariant() IState
}
