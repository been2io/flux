package execute

import (
	"context"
	"errors"
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/plan"
)

func init() {
	RegisterSource("stage", createStageSource)

}

var CreateReader func(spec flux.Spec) (flux.TableIterator, error)

func createStageSource(s plan.ProcedureSpec, id DatasetID, a Administration) (Source, error) {
	spec := s.(*plan.StageProcedureSpec)
	if spec == nil {
		return nil, errors.New("no spec with stage")
	}
	t := &StageSource{
		id:   id,
		spec: spec.Spec,
	}
	return t, nil
}

type StageSource struct {
	id     DatasetID
	ts     []Transformation
	spec   flux.Spec
	bounds Bounds
}

func (rs *StageSource) RunTables(ctx context.Context) error {
	tables, err := CreateReader(rs.spec)
	if err != nil {
		return err
	}
	err = tables.Do(func(table flux.Table) error {
		return rs.processTable(ctx, table)
	})
	if err != nil {
		return err
	}
	watermark := rs.bounds.Stop
	for _, t := range rs.ts {
		if err := t.UpdateWatermark(rs.id, watermark); err != nil {
			return err
		}
	}
	return err
}
func (rs *StageSource) Run(ctx context.Context) {
	err := rs.RunTables(ctx)
	for _, t := range rs.ts {
		t.Finish(rs.id, err)
	}
}

func (rs *StageSource) AddTransformation(t Transformation) {
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
	bufTable, err := CopyTable(tbl)
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
