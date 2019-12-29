package stage

import (
	"context"
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/execute"
	"github.com/influxdata/flux/plan"
)

const StageKind = "stage"

var CreateReader func() (flux.TableIterator, error)

func init() {
	flux.RegisterOpSpec(StageKind, newStageOp)
	plan.RegisterProcedureSpec(StageKind, createStageProcedureSpec, StageKind)
	execute.RegisterSource(StageKind, createStageSource)

}
func newStageOp() flux.OperationSpec {
	return StageOperationSpec{}
}

type StageOperationSpec struct {
	Spec *flux.Spec
}

func (s StageOperationSpec) AddOperation(operation *flux.Operation) {
	s.Spec.Operations = append(s.Spec.Operations, operation)
}
func (s StageOperationSpec) AddEdge(e flux.Edge) {
	s.Spec.Edges = append(s.Spec.Edges, e)
}
func (StageOperationSpec) Kind() flux.OperationKind {
	return StageKind
}

type stageProcedureSpec struct {
	Spec *flux.Spec
}

func (spec stageProcedureSpec) Cost(inStats []plan.Statistics) (cost plan.Cost, outStats plan.Statistics) {
	return plan.Cost{}, plan.Statistics{}
}

func createStageProcedureSpec(qs flux.OperationSpec, pa plan.Administration) (plan.ProcedureSpec, error) {
	o := qs.(*StageOperationSpec)

	return &stageProcedureSpec{
		Spec: o.Spec,
	}, nil
}
func (spec stageProcedureSpec) Kind() plan.ProcedureKind {
	return StageKind
}

func (spec stageProcedureSpec) Copy() plan.ProcedureSpec {
	return &stageProcedureSpec{
		Spec: spec.Spec,
	}
}

type stageDataset struct {
	tables chan flux.Table
	ts     []execute.Transformation
	id     execute.DatasetID
}

func (rs *stageDataset) AddTransformation(t execute.Transformation) {
	rs.ts = append(rs.ts, t)
}

func (rs *stageDataset) RetractTable(key flux.GroupKey) error {
	panic("implement me")
}

func (rs *stageDataset) UpdateProcessingTime(t execute.Time) error {
	panic("implement me")
}

func (rs *stageDataset) UpdateWatermark(mark execute.Time) error {
	panic("implement me")
}

func (rs *stageDataset) Finish(error) {
	panic("implement me")
}

func (rs *stageDataset) SetTriggerSpec(t plan.TriggerSpec) {
	panic("implement me")
}

func (rs *stageDataset) Add(table flux.Table) {
	rs.tables <- table
}
func createStageSource(s plan.ProcedureSpec, id execute.DatasetID, a execute.Administration) (execute.Source, error) {
	t := &StageSource{
		id: id,
	}
	return t, nil
}

type StageSource struct {
	id execute.DatasetID
	ts []execute.Transformation
}

func (rs *StageSource) Run(ctx context.Context) {
	tables, err := CreateReader()
	if err != nil {
		panic(err)
	}
	err = tables.Do(func(table flux.Table) error {
		return rs.processTable(ctx, table)
	})
	if err != nil {
		panic(err)
	}
}

func (rs *StageSource) AddTransformation(t execute.Transformation) {
	rs.ts = append(rs.ts, t)
}
func (s *StageSource) processTable(ctx context.Context, tbl flux.Table) error {
	if len(s.ts) == 0 {
		tbl.Done()
		return nil
	} else if len(s.ts) == 1 {
		return s.ts[0].Process(s.id, tbl)
	}

	// There is more than one transformation so we need to
	// copy the table for each transformation.
	bufTable, err := execute.CopyTable(tbl)
	if err != nil {
		return err
	}
	defer bufTable.Done()

	for _, t := range s.ts {
		if err := t.Process(s.id, bufTable.Copy()); err != nil {
			return err
		}
	}
	return nil
}
