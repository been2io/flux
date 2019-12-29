package flux
func (spec *Spec) DetermineParentsChildrenAndRoots()(parents, children map[OperationID][]*Operation, roots []*Operation, _ error) {
	return spec.determineParentsChildrenAndRoots()
}
