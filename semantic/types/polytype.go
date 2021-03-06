package types

import (
	"strings"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/influxdata/flux/codes"
	"github.com/influxdata/flux/internal/errors"
	"github.com/influxdata/flux/semantic/internal/fbsemantic"
)

// PolyType represents a polytype.  This struct is a thin wrapper around
// Go code generated by the FlatBuffers compiler.
type PolyType struct {
	fb *fbsemantic.PolyType
}

// NewPolyType returns a new polytype given a flatbuffers polytype.
func NewPolyType(fb *fbsemantic.PolyType) (*PolyType, error) {
	if fb == nil {
		return nil, errors.New(codes.Internal, "got nil fbsemantic.polytype")
	}
	return &PolyType{fb: fb}, nil
}

// NumVars returns the number of type variables in this polytype.
func (pt *PolyType) NumVars() int {
	return pt.fb.VarsLength()
}

// Var returns the type variable at ordinal position i.
func (pt *PolyType) Var(i int) (*fbsemantic.Var, error) {
	if i < 0 || i >= pt.NumVars() {
		return nil, errors.Newf(codes.Internal, "request for polytype var out of bounds: %v in %v", i, pt.NumVars())
	}
	v := new(fbsemantic.Var)
	if !pt.fb.Vars(v, i) {
		return nil, errors.Newf(codes.Internal, "missing var")
	}
	return v, nil
}

// NumConstraints returns the number of kind constraints in this polytype.
func (pt *PolyType) NumConstraints() int {
	return pt.fb.ConsLength()
}

// Constraint returns the constraint at ordinal position i.
func (pt *PolyType) Constraint(i int) (*fbsemantic.Constraint, error) {
	if i < 0 || i >= pt.NumConstraints() {
		return nil, errors.Newf(codes.Internal, "request for constraint out of bounds: %v in %v", i, pt.NumConstraints())
	}
	c := new(fbsemantic.Constraint)
	if !pt.fb.Cons(c, i) {
		return nil, errors.Newf(codes.Internal, "missing constraint")
	}
	return c, nil

}

// Expr returns the monotype expression for this polytype.
func (pt *PolyType) Expr() (*MonoType, error) {
	tbl := new(flatbuffers.Table)
	if !pt.fb.Expr(tbl) {
		return nil, errors.New(codes.Internal, "missing a polytype expr")
	}

	return NewMonoType(tbl, pt.fb.ExprType())
}

// String returns a string representation for this polytype.
func (pt *PolyType) String() string {
	var sb strings.Builder

	sb.WriteString("forall [")
	needComma := false
	for i := 0; i < pt.NumVars(); i++ {
		v, err := pt.Var(i)
		if err != nil {
			return "<" + err.Error() + ">"
		}
		if needComma {
			sb.WriteString(", ")
		} else {
			needComma = true
		}
		mt := monoTypeFromVar(v)
		sb.WriteString(mt.String())
	}
	sb.WriteString("] ")

	needWhere := true
	for i := 0; i < pt.NumConstraints(); i++ {
		cons, err := pt.Constraint(i)
		if err != nil {
			return "<" + err.Error() + ">"
		}
		tv := cons.Tvar(nil)
		k := cons.Kind()

		if needWhere {
			sb.WriteString("where ")
		} else {
			needWhere = false
		}
		mtv := monoTypeFromVar(tv)
		sb.WriteString(mtv.String())
		sb.WriteString(": ")
		sb.WriteString(fbsemantic.EnumNamesKind[k])

		if i < pt.NumConstraints()-1 {
			sb.WriteString(", ")
		} else {
			sb.WriteString(" ")
		}
	}

	mt, err := pt.Expr()
	if err != nil {
		return "<" + err.Error() + ">"
	}
	sb.WriteString(mt.String())

	return sb.String()
}
