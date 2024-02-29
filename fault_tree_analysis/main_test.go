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

	if tree.Val != 0.03977951284000014 {
		t.Errorf("Expected 0.03977951284000014, got %f", tree.Val)
	}
}

func TestCalcEffectivity(t *testing.T) {
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

	criteria := map[string][][]float64{
		"A1": {
			{0.01, 10000.0},
			{0.001, 100000.0},
			{0.0005, 110000.0},
			{0.0004, 200000.0},
			{0.0003, 400000.0},
		},
		"A2": {
			{0.0001, 100000.0},
			{0.00005, 1000000.0},
			{0.00004, 2000000.0},
			{0.00002, 2500000.0},
			{0.00001, 3000000.0},
		},
		"B1": {
			{0.004, 15000.0},
			{0.003, 20000.0},
			{0.002, 25000.0},
			{0.001, 30000.0},
			{0.0005, 40000.0},
		},
		"B2": {
			{0.0004, 200000.0},
			{0.0003, 250000.0},
			{0.0002, 400000.0},
			{0.0001, 700000.0},
			{0.00005, 800000.0},
		},
	}	

	RMax := 0.1
	EMax := 100000000.0

	effectivityResult := calcEffectivity(criteria, RMax, EMax, tree)

	if effectivityResult.VAL != 0.00030999872446513255 {
		t.Errorf("Expected 0.00030999872446513255, got %f", effectivityResult.VAL)
	}
}
