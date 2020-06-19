package semantic

import "encoding/json"

func (n *Nature) UnmarshalJSON(b []byte) error {
	var i int
	if err := json.Unmarshal(b, &i); err != nil {
		return err
	}
	*n = Nature(i)
	return nil
}
func (n *Nature)  MarshalJSON() ([]byte, error) {
	i:=int(*n)
	return json.Marshal(i)
}
