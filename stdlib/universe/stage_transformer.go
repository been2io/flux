package universe

import (
	"github.com/influxdata/flux/execute"
	"github.com/influxdata/flux/plan"
)

type createStageTransformerPhysicRule struct {

}

func (c createStageTransformerPhysicRule) Name() string {
	return execute.StageTransformerKind
}

func (c createStageTransformerPhysicRule) Pattern() plan.Pattern {
	return plan.Pat(windowAggregate)
}

func (c createStageTransformerPhysicRule) Rewrite(node plan.Node) (plan.Node, bool, error) {
	panic("implement me")
}
