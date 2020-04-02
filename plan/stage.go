package plan

import (
	"github.com/influxdata/flux"
)

const StageKind = "stage"

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

type visitor struct {
	relations map[flux.OperationID][]*flux.Operation
	skipKinds []flux.OperationKind
	last      *flux.Operation
}

func (v *visitor) walk(operation *flux.Operation, f func(first, second *flux.Operation)) {
	if v.last == nil {
		v.last = operation
	}
	parent := operation
	skipped := false
	for _, skip := range v.skipKinds {
		if skip == v.last.Spec.Kind() {
			skipped = true
			break
		}
	}
	if skipped {
		parent = v.last
	}
	for _, op := range v.relations[operation.ID] {
		f(parent, op)
		v.walk(op, f)
	}
}
func edge(parent *flux.Operation, child *flux.Operation) flux.Edge {
	return flux.Edge{Parent: parent.ID, Child: child.ID}
}
func (sp StagePlanner) Plan(spec *flux.Spec) (*flux.Spec, error) {
	_, children, roots, err := spec.DetermineParentsChildrenAndRoots()
	if err != nil {
		return nil, err
	}
	var markedOp []*flux.Operation
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
			markedOp = append(markedOp, n)
			stageSpec.AddOperation(n)
			stageSpec.AddEdge(edge(p, n))
		})
		root.Spec = stageSpec
	}
	skipped := []flux.OperationKind{flux.OperationKind("group"), flux.OperationKind("filter"),flux.OperationKind("window")}

	for _, op := range markedOp {
		for _, skip := range skipped {
			if skip == op.Spec.Kind() {
				var ops []*flux.Operation
				var edges []flux.Edge
				//rm operations
				for _, o := range spec.Operations {
					if o.ID != op.ID {
						ops = append(ops, o)
					}
				}
				// rm edge
				c := spec.Children(op.ID)
				ps := spec.Parents(op.ID)

				for _, edge := range spec.Edges {
					if edge.Child != op.ID && edge.Parent != op.ID {
						edges = append(edges, edge)
					}
				}
				for _, p := range ps {
					for _, child := range c {
						edges = append(edges, edge(p, child))
					}
				}
				spec.Operations = ops
				spec.Edges = edges
				if err := spec.Validate(); err != nil {
					return nil,err
				}
			}
		}
	}

	return spec, nil

}
