// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package fbsemantic

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type ConditionalExpression struct {
	_tab flatbuffers.Table
}

func GetRootAsConditionalExpression(buf []byte, offset flatbuffers.UOffsetT) *ConditionalExpression {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &ConditionalExpression{}
	x.Init(buf, n+offset)
	return x
}

func (rcv *ConditionalExpression) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *ConditionalExpression) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *ConditionalExpression) Loc(obj *SourceLocation) *SourceLocation {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		x := rcv._tab.Indirect(o + rcv._tab.Pos)
		if obj == nil {
			obj = new(SourceLocation)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *ConditionalExpression) TestType() byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetByte(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ConditionalExpression) MutateTestType(n byte) bool {
	return rcv._tab.MutateByteSlot(6, n)
}

func (rcv *ConditionalExpression) Test(obj *flatbuffers.Table) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		rcv._tab.Union(obj, o)
		return true
	}
	return false
}

func (rcv *ConditionalExpression) AlternateType() byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		return rcv._tab.GetByte(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ConditionalExpression) MutateAlternateType(n byte) bool {
	return rcv._tab.MutateByteSlot(10, n)
}

func (rcv *ConditionalExpression) Alternate(obj *flatbuffers.Table) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(12))
	if o != 0 {
		rcv._tab.Union(obj, o)
		return true
	}
	return false
}

func (rcv *ConditionalExpression) ConsequentType() byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(14))
	if o != 0 {
		return rcv._tab.GetByte(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ConditionalExpression) MutateConsequentType(n byte) bool {
	return rcv._tab.MutateByteSlot(14, n)
}

func (rcv *ConditionalExpression) Consequent(obj *flatbuffers.Table) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(16))
	if o != 0 {
		rcv._tab.Union(obj, o)
		return true
	}
	return false
}

func (rcv *ConditionalExpression) TypType() byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(18))
	if o != 0 {
		return rcv._tab.GetByte(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ConditionalExpression) MutateTypType(n byte) bool {
	return rcv._tab.MutateByteSlot(18, n)
}

func (rcv *ConditionalExpression) Typ(obj *flatbuffers.Table) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(20))
	if o != 0 {
		rcv._tab.Union(obj, o)
		return true
	}
	return false
}

func ConditionalExpressionStart(builder *flatbuffers.Builder) {
	builder.StartObject(9)
}
func ConditionalExpressionAddLoc(builder *flatbuffers.Builder, loc flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(loc), 0)
}
func ConditionalExpressionAddTestType(builder *flatbuffers.Builder, testType byte) {
	builder.PrependByteSlot(1, testType, 0)
}
func ConditionalExpressionAddTest(builder *flatbuffers.Builder, test flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(2, flatbuffers.UOffsetT(test), 0)
}
func ConditionalExpressionAddAlternateType(builder *flatbuffers.Builder, alternateType byte) {
	builder.PrependByteSlot(3, alternateType, 0)
}
func ConditionalExpressionAddAlternate(builder *flatbuffers.Builder, alternate flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(4, flatbuffers.UOffsetT(alternate), 0)
}
func ConditionalExpressionAddConsequentType(builder *flatbuffers.Builder, consequentType byte) {
	builder.PrependByteSlot(5, consequentType, 0)
}
func ConditionalExpressionAddConsequent(builder *flatbuffers.Builder, consequent flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(6, flatbuffers.UOffsetT(consequent), 0)
}
func ConditionalExpressionAddTypType(builder *flatbuffers.Builder, typType byte) {
	builder.PrependByteSlot(7, typType, 0)
}
func ConditionalExpressionAddTyp(builder *flatbuffers.Builder, typ flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(8, flatbuffers.UOffsetT(typ), 0)
}
func ConditionalExpressionEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
