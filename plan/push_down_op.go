package plan

import (
	"github.com/influxdata/flux"
)

func IsPushDownOp(o *flux.Operation) bool {
	switch o.Spec.Kind() {
	case "range":
		return true
	case "filter":
		return true
	case "window":
	case "first":
	case "last":
	case "sum":
	case "sample":
	case "group":
		return true
	case "from":
	case "influxDBFrom":
	default:
		return false
	}
	return false
}
