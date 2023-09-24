package brainblock

import "gonum.org/v1/gonum/mat"

type Piece struct {
	BaseType   BaseType
	Shape      *mat.Dense
	Reversible bool
}

type BaseType string

const (
	SQUARE  BaseType = "SQUARE"
	HEXAGON          = "HEXAGON"
)
