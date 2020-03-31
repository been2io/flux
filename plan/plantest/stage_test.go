package plantest

import (
	"github.com/influxdata/flux"
	plan2 "github.com/influxdata/flux/plan"
	"github.com/influxdata/flux/stdlib/influxdata/influxdb"
	"github.com/influxdata/flux/stdlib/universe"
	"testing"
	"time"
)

func TestStagePlanner_Plan(t *testing.T) {
	plan := plan2.StagePlanner{}
	expQ := flux.Spec{
		Now:time.Now(),
		Operations: []*flux.Operation{
			{
				ID: "from",
				Spec: &influxdb.FromOpSpec{
					Bucket: "mybucket",
				},
			},
			{
				ID: "range",
				Spec: &universe.RangeOpSpec{
					Start: flux.Time{
						Relative:   -4 * time.Hour,
						IsRelative: true,
					},
					Stop: flux.Time{
						IsRelative: true,
					},
				},
			},
			{
				ID:   "filter",
				Spec: &universe.FilterOpSpec{},
			},
			{
				ID:   "group",
				Spec: &universe.GroupOpSpec{},
			},
			{
				ID:   "sum",
				Spec: &universe.SumOpSpec{},
			},
		},
		Edges: []flux.Edge{
			{Parent: "from", Child: "range"},
			{Parent: "range", Child: "filter"},
			{Parent: "filter", Child: "group"},
			{Parent: "group", Child: "sum"},
		},
	}
	sp, err := plan.Plan(&expQ)
	if err != nil {
		panic(err)
	}
	if len(sp.Operations) !=3{
		t.Error("fail")
	}

}
