package main

import "testing"

func TestCalcNoTree(t *testing.T) {

	tree := N{"TC", OR, 0.0, []N{
		N{"A", OR, 0.0, []N{
			N{"A1", NO, 0.03, []N{}},
			N{"A2", NO, 0.01, []N{}},
		}},
		N{"T1", AND, 0.0, []N{
			N{"B", AND, 0.0, []N{
				N{"B1", NO, 0.03, []N{}},
				N{"B2", NO, 0.04, []N{}},
			}},
			N{"C", OR, 0.0, []N{
				N{"C1", NO, 0.02, []N{}},
				N{"C2", NO, 0.05, []N{}},
			}},
		}},
	}}

	calcNoTree(&tree)

	if tree.Val != 0.039779484 {
		t.Errorf("Expected 0.039779484, got %f", tree.Val)
	}

	if tree.Children[1].Val != 0.000082800005 {
		t.Errorf("Expected 0.000082800005, got %f", tree.Children[1].Val)
	}
}
