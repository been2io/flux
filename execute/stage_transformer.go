package execute

import (
	"context"
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/memory"
	"github.com/influxdata/flux/plan"
	uuid "github.com/satori/go.uuid"
	"go.uber.org/zap"
)

const StageTransformerKind = "stageTransformer"



type stageTransformerPhysicSpec struct {
	pl *plan.Spec
}

func (s stageTransformerPhysicSpec) RetractTable(key flux.GroupKey) error {
	panic("implement me")
}

func (s stageTransformerPhysicSpec) UpdateProcessingTime(t Time) error {
	panic("implement me")
}

func (s stageTransformerPhysicSpec) UpdateWatermark(mark Time) error {
	panic("implement me")
}

func (s stageTransformerPhysicSpec) Finish(err error) {
	panic("implement me")
}

func (s stageTransformerPhysicSpec) SetTriggerSpec(t plan.TriggerSpec) {
	panic("implement me")
}

func (s stageTransformerPhysicSpec) Kind() plan.ProcedureKind {
	return StageTransformerKind
}

func (s stageTransformerPhysicSpec) Copy() plan.ProcedureSpec {
	return s
}

func (s stageTransformerPhysicSpec) Cost(inStats []plan.Statistics) (cost plan.Cost, outStats plan.Statistics) {
	return plan.Cost{}, plan.Statistics{}
}

type stageTransformer struct {
	spec      stageTransformerPhysicSpec
	successor TransformationSet
	ds        dataset
	es        *executionState
	dg        DatasetGroup
}

func (s *stageTransformer) createExecutionState() error {
	es := &executionState{
		p:         s.spec.pl,
		alloc:     &memory.Allocator{},
		resources: flux.ResourceManagement{},
		results:   make(map[string]flux.Result),
		// TODO(nathanielc): Have the planner specify the dispatcher throughput
		dispatcher: newPoolDispatcher(10, zap.NewNop()),
	}
	v := &createExecutionNodeVisitor{
		ctx:   context.Background(),
		es:    es,
		nodes: make(map[plan.Node]Node),
	}

	if err := s.spec.pl.BottomUpWalk(v.Visit); err != nil {
		return err
	}
	return nil
}
func (s stageTransformer) RetractTable(id DatasetID, key flux.GroupKey) error {
	panic("implement me")
}

func (s stageTransformer) Process(id DatasetID, tbl flux.Table) error {
	newId := uuid.NewV5(uuid.UUID(id), tbl.Key().String())
	ds := NewPassthroughDataset(DatasetID(newId))
	for _, source := range s.es.sources {
		tr := source.(Transformation)
		t := newConsecutiveTransport(s.es.dispatcher, tr)
		ds.AddTransformation(t)
	}
	s.dg.Add(id, ds)
	return ds.Process(tbl)
}

func (s stageTransformer) UpdateWatermark(id DatasetID, t Time) error {
	return s.dg.Foreach(id, func(d Dataset) error {
		return d.UpdateWatermark(t)
	})
}

func (s stageTransformer) UpdateProcessingTime(id DatasetID, t Time) error {
	return s.dg.Foreach(id, func(d Dataset) error {
		return d.UpdateProcessingTime(t)
	})
}

func (s stageTransformer) Finish(id DatasetID, err error) {
	s.dg.Foreach(id, func(d Dataset) error {
		d.Finish(err)
		return nil
	})
}

type DatasetGroup struct {
	ds map[DatasetID][]Dataset
}

func (dg *DatasetGroup) Add(id DatasetID, d Dataset) {
	dg.ds[id] = append(dg.ds[id], d)
}
func (dg *DatasetGroup) Foreach(id DatasetID, f func(d Dataset) error) error {
	for _, d := range dg.ds[id] {
		if err := f(d); err != nil {
			return err
		}
	}
	return nil
}
