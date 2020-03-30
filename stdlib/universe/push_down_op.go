package universe

import (
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/plan"
	"github.com/influxdata/flux/stdlib/influxdata/influxdb"
)

func init() {
	plan.IsPushDownOp = isPushDownOp
}
func isPushDownOp(o *flux.Operation) bool {

	switch o.Spec.Kind() {
	case RangeKind:
	case FilterKind:
	case WindowKind:
	case FirstKind:
	case LastKind:
	case SumKind:
	case SampleKind:
	case GroupKind:
	case influxdb.FromKind:
	case "influxDBFrom":

	default:
		return false
	}
	return true
}
