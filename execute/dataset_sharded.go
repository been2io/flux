package execute

import (
	"github.com/influxdata/flux"
	"github.com/influxdata/flux/plan"
)

type ConcurrentDataset struct {
	id  DatasetID
	ts  TransformationSet
	idx int
}

func NewConcurrentDataset(id DatasetID) *ConcurrentDataset {
	return &ConcurrentDataset{id: id}
}

func (d *ConcurrentDataset) AddTransformation(t Transformation) {
	d.ts = append(d.ts, t)
}

func (d *ConcurrentDataset) Process(tbl flux.Table) error {
	i := d.idx % len(d.ts)
	d.idx++
	return d.ts[i].Process(d.id, tbl)
}

func (d *ConcurrentDataset) RetractTable(key flux.GroupKey) error {
	return d.ts.RetractTable(d.id, key)
}

func (d *ConcurrentDataset) UpdateProcessingTime(t Time) error {
	return d.ts.UpdateProcessingTime(d.id, t)
}

func (d *ConcurrentDataset) UpdateWatermark(mark Time) error {
	return d.ts.UpdateWatermark(d.id, mark)
}

func (d *ConcurrentDataset) Finish(err error) {
	d.ts.Finish(d.id, err)
}

func (d *ConcurrentDataset) SetTriggerSpec(t plan.TriggerSpec) {
}

type ConcurrentIndicatorTransformer struct {
	ds ConcurrentDataset
}

func (c *ConcurrentIndicatorTransformer) RetractTable(id DatasetID, key flux.GroupKey) error {
	return c.ds.RetractTable(key)
}

func (c *ConcurrentIndicatorTransformer) Process(id DatasetID, tbl flux.Table) error {
	return c.ds.Process(tbl)
}

func (c *ConcurrentIndicatorTransformer) UpdateWatermark(id DatasetID, t Time) error {
	return c.ds.UpdateWatermark(t)
}

func (c *ConcurrentIndicatorTransformer) UpdateProcessingTime(id DatasetID, t Time) error {
	return c.ds.UpdateProcessingTime(t)
}

func (c *ConcurrentIndicatorTransformer) Finish(id DatasetID, err error) {
	c.ds.Finish(err)
}
