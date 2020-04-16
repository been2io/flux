package universe_test

import (
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/execute"
	"github.com/influxdata/flux/plan"
	_ "github.com/influxdata/flux/plan"
	"github.com/influxdata/flux/querytest"
	"github.com/influxdata/flux/stdlib/influxdata/influxdb"
	"github.com/influxdata/flux/stdlib/universe"
	"testing"
	"time"
)

func Test_RateQuery(t *testing.T) {
	tests := []querytest.NewQueryTestCase{
		{
			Name: "rate test",
			Raw:  `from(bucket:"mybucket") |> range(start:-4h, stop:-2h) |> rate(columns:["host"],every:1m)`,
			Want: &flux.Spec{
				Operations: []*flux.Operation{
					{
						ID: "from0",
						Spec: &influxdb.FromOpSpec{
							Bucket: "mybucket",
						},
					},
					{
						ID: "range1",
						Spec: &universe.RangeOpSpec{
							Start: flux.Time{
								Relative:   -4 * time.Hour,
								IsRelative: true,
							},
							Stop: flux.Time{
								Relative:   -2 * time.Hour,
								IsRelative: true,
							},
							TimeColumn:  "_time",
							StartColumn: "_start",
							StopColumn:  "_stop",
						},
					},
					{
						ID:   "derivative2",
						Spec: &universe.DerivativeOpSpec{Unit: flux.ConvertDuration(time.Minute), NonNegative: true, Columns: []string{"_value"}, TimeColumn: "_time"},
					},
					{
						ID: "group3",
						Spec: &universe.GroupOpSpec{
							Mode:    "by",
							Columns: []string{"host"},
						},
					},
					{
						ID: "sum4",
						Spec: &universe.SumOpSpec{
							AggregateConfig: execute.DefaultAggregateConfig,
						},
					},
					{
						ID:   "stage5",
						Spec: &plan.StageOperationSpec{},
					},
					{
						ID:   "sum6",
						Spec: &universe.SumOpSpec{AggregateConfig: execute.AggregateConfig{Columns: []string{"_value"}}},
					},
				},
				Edges: []flux.Edge{
					{Parent: "from0", Child: "range1"},
					{Parent: "range1", Child: "derivative2"},
					{Parent: "derivative2", Child: "group3"},
					{Parent: "group3", Child: "sum4"},
					{Parent: "sum4", Child: "stage5"},
					{Parent: "stage5", Child: "sum6"},
				},
			},
		},
	}
	for _, tc := range tests {
		tc := tc
		t.Run(tc.Name, func(t *testing.T) {
			querytest.NewQueryTestHelper(t, tc)
		})
	}
}
