package universe

import (
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/stage"
	"github.com/influxdata/flux/stdlib/influxdata/influxdb"
)

func init() {
	stage.IsPushDownOp = isPushDownOp
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

	default:
		return false
	}
	return true
}
