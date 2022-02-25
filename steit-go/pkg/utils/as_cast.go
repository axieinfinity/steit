package utils

import "github.com/axieinfinity/steit-go/pkg/state"

func AsIState(data interface{}) state.IState {
	val, ok := data.(state.IState)
	if !ok {
		return nil
	}
	return val
}
