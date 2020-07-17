package plan

const mergeConcurrentOperation = "mergeConcurrentOperation"
type mergeConcurrentOperationRule struct {

}

func (m mergeConcurrentOperationRule) Name() string {
	panic("implement me")
}

func (m mergeConcurrentOperationRule) Pattern() Pattern {
	panic("implement me")
}

func (m mergeConcurrentOperationRule) Rewrite(node Node) (Node, bool, error) {
	panic("implement me")
}

