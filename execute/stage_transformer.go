package execute

import (
	"context"
	"fmt"
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/plan"
	uuid "github.com/satori/go.uuid"
)

const StageTransformerKind = "stageTransformer"

type stageTransformerPhysicSpec struct {
	n plan.Node
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
	node      plan.Node
	successor *PassthroughDataset
	ds        *ConcurrentDataset
	es        *executionState
}

func createStageTransformer(id DatasetID, es *executionState, node plan.Node, concurrency int) (Transformation, Dataset, error) {
	succ := NewPassthroughDataset(id)
	ds := NewConcurrentDataset(id, concurrency)
	t := &stageTransformer{
		node:      node,
		successor: succ,
		ds:        ds,
		es:        es,
	}
	return t, succ, nil
}
func (s *stageTransformer) createExecutionState(ctx context.Context, pred Dataset, key string) error {
	node := s.node
	ppn, ok := node.(*plan.PhysicalPlanNode)
	if !ok {
		return fmt.Errorf("cannot execute plan node of type %T", node)
	}
	spec := node.ProcedureSpec()
	kind := spec.Kind()
	id := DatasetIDFromNodeID(node.ID())
	newId := uuid.NewV5(uuid.UUID(id), key)

	createTransformationFn, ok := procedureToTransformation[kind]
	if !ok {
		return fmt.Errorf("unsupported procedure %v", kind)
	}
	var streamContext streamContext
	if node.Bounds() != nil {
		streamContext.bounds = &Bounds{
			Start: node.Bounds().Start,
			Stop:  node.Bounds().Stop,
		}
	}
	ec := executionContext{
		ctx:           ctx,
		es:            s.es,
		parents:       make([]DatasetID, len(node.Predecessors())),
		streamContext: streamContext,
	}
	tr, ds, err := createTransformationFn(DatasetID(newId), DiscardingMode, spec, ec)
	if err != nil {
		return err
	}
	if ppn.TriggerSpec == nil {
		ppn.TriggerSpec = plan.DefaultTriggerSpec
	}
	ds.SetTriggerSpec(ppn.TriggerSpec)
	transport := newConsecutiveTransport(s.es.dispatcher, tr)
	pred.AddTransformation(transport)
	for _, t := range s.successor.ts {
		ds.AddTransformation(t)
	}

	return nil
}
func (s stageTransformer) RetractTable(id DatasetID, key flux.GroupKey) error {
	panic("implement me")
}

func (s *stageTransformer) Process(id DatasetID, tbl flux.Table) error {
	if s.ds.Size() < s.ds.Cap() {
		err := s.createExecutionState(context.Background(), s.ds, tbl.Key().String())
		if err != nil {
			return err
		}
	}
	return s.ds.Process(tbl)
}

func (s stageTransformer) UpdateWatermark(id DatasetID, t Time) error {
	return s.ds.UpdateWatermark(t)
}

func (s stageTransformer) UpdateProcessingTime(id DatasetID, t Time) error {
	return s.ds.UpdateProcessingTime(t)
}

func (s stageTransformer) Finish(id DatasetID, err error) {
	s.ds.Finish(err)
}
