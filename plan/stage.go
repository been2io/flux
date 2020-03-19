package plan

import (
	"fmt"
	"github.com/influxdata/flux"
)

const StageKind = "stage"

var CreateReader func(spec flux.Spec) (flux.TableIterator, error)

func init() {
	flux.RegisterOpSpec(StageKind, newStageOp)
	RegisterProcedureSpec(StageKind, createStageProcedureSpec, StageKind)

}
func newStageOp() flux.OperationSpec {
	return StageOperationSpec{}
}

type StageOperationSpec struct {
	Spec flux.Spec
}

func (s *StageOperationSpec) AddOperation(operation *flux.Operation) {
	o := *operation
	s.Spec.Operations = append(s.Spec.Operations, &o)
}
func (s *StageOperationSpec) AddEdge(e flux.Edge) {
	s.Spec.Edges = append(s.Spec.Edges, e)
}
func (StageOperationSpec) Kind() flux.OperationKind {
	return StageKind
}

type StageProcedureSpec struct {
	Spec flux.Spec
}

func (spec StageProcedureSpec) Cost(inStats []Statistics) (cost Cost, outStats Statistics) {
	return Cost{}, Statistics{}
}

func createStageProcedureSpec(qs flux.OperationSpec, pa Administration) (ProcedureSpec, error) {
	o := qs.(*StageOperationSpec)

	return &StageProcedureSpec{
		Spec: o.Spec,
	}, nil
}
func (spec StageProcedureSpec) Kind() ProcedureKind {
	return StageKind
}

func (spec StageProcedureSpec) Copy() ProcedureSpec {
	return &StageProcedureSpec{
		Spec: spec.Spec,
	}
}

type StagePlanner struct {
}

type stageMarkVisitor struct {
	children map[flux.OperationID][]*flux.Operation
}

func (v *stageMarkVisitor) walk(op *flux.Operation, f func(p, n *flux.Operation)) {
	ops := v.children[op.ID]
	for _, ch := range ops {
		if IsPushDownOp(ch) {
			f(op, ch)
			v.walk(ch, f)
		}

	}

}

var IsPushDownOp func(op *flux.Operation) bool

type visitor struct {
	relations map[flux.OperationID][]*flux.Operation
}

func (v *visitor) walk(operation *flux.Operation, f func(first, second *flux.Operation)) {
	for _, op := range v.relations[operation.ID] {
		f(operation, op)
	}
}

func edge(parent *flux.Operation, child *flux.Operation) flux.Edge {
	return flux.Edge{Parent: parent.ID, Child: child.ID}
}
func (sp StagePlanner) Plan(spec *flux.Spec) (*flux.Spec, error) {
	parents, children, roots, err := spec.DetermineParentsChildrenAndRoots()
	if err != nil {
		return nil, err
	}
	var stageMarks = make(map[*flux.Operation]struct{})
	v := stageMarkVisitor{
		children: children,
	}
	for _, root := range roots {
		stageSpec := &StageOperationSpec{
			Spec: flux.Spec{
				Now:       spec.Now,
				Resources: spec.Resources,
			},
		}
		stageSpec.AddOperation(root)
		v.walk(root, func(p, n *flux.Operation) {
			stageSpec.AddOperation(n)
			stageSpec.AddEdge(edge(p, n))
		})
		root.Spec = stageSpec
	}
	return spec, nil
	//build new spec

	new := &flux.Spec{
		Resources: spec.Resources,
	}
	i := 0

	for markedOp, _ := range stageMarks {
		//build stage spec
		i++

		stageOpSpec := &StageOperationSpec{

			Spec: flux.Spec{
				Resources: spec.Resources,
			},
		}

		stageOp := &flux.Operation{
			ID:   flux.OperationID(fmt.Sprintf("stage%v", i)),
			Spec: stageOpSpec,
		}
		v := visitor{relations: parents}
		stageOpSpec.AddOperation(markedOp)
		stageOpSpec.AddEdge(edge(markedOp, stageOp))

		v.walk(markedOp, func(first, second *flux.Operation) {
			stageOpSpec.AddOperation(second)
			stageOpSpec.AddEdge(flux.Edge{Parent: second.ID, Child: first.ID})
		})

		//build new spec
		new.Operations = append(new.Operations, stageOp)
		new.Operations = append(new.Operations, markedOp)
		v = visitor{
			relations: children,
		}
		v.walk(markedOp, func(first, second *flux.Operation) {
			new.Operations = append(new.Operations, second)
			new.Edges = append(new.Edges, edge(first, second))
		})

	}
	return new, nil

}
