// Fault Tree Analysis
// https://www.youtube.com/watch?v=Eq8-m6Faobo

package main

import (
	"time"
	"sort"
	"encoding/json"
	"fmt"
	"log"
)

type Cond int

const (
	NO Cond = iota
	AND
	OR
)

type N struct {
	Name     string  `json:"name"`
	Cond     Cond    `json:"cond"`
	Val      float64 `json:"val"`
	Children []N     `json:"children"`
}

type EffectivityResult struct {
	VAL float64
	SUMR float64
	SUME float64
	VARIANT [][]float64
}

func main() {

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

	fmt.Println("Tree before calc")
	printTree(tree)

	calcNoTree(&tree)
	fmt.Println("Tree after calc")
	printTree(tree)

	fmt.Println("Calc effectivity")

	// 10000.0 + 100000.0 + 15000.0 + 200000.0 = 325000.0
	// 0.01 + 0.0001 + 0.004 + 0.0004 = 0.0145
	// 400000.0 + 3000000.0 + 40000.0 + 800000.0 = 4240000.0
	// 0.0003 + 0.00001 + 0.0005 + 0.00005 = 0,00086
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

	// RMax := 0.000001
	RMax := 0.1
	EMax := 100000000.0

	effectivityResult := calcEffectivity(criteria, RMax, EMax, tree)

	fmt.Println("Effectivity result", effectivityResult)
}

func printTree(tree N) {
	empJSON, err := json.MarshalIndent(tree, "", "  ")
	if err != nil {
		log.Fatalf(err.Error())
	}
	fmt.Println(string(empJSON))
}

func calcNoTree(tree *N) {
	var walk func(item *N)

	walk = func(item *N) {
		if len(item.Children) == 0 {
			return
		}
		for i := 0; i < len(item.Children); i++ {
			walk(&item.Children[i])
		}

		var val float64 = 1.0
		if item.Cond == AND {
			for _, child := range item.Children {
				val *= child.Val
			}

			item.Val = val
		}
		if item.Cond == OR {
			for _, child := range item.Children {
				val *= (1.0 - child.Val)
			}
			item.Val = 1 - val
		}

	}
	walk(tree)
}

func calcEffectivity(criteria map[string][][]float64, RMax float64, EMax float64, tree N) EffectivityResult {
	start := time.Now()

	clonedTree := tree

	keys := make([]string, 0)
	for k, _ := range criteria {
			keys = append(keys, k)
	}
	sort.Strings(keys)

	var listOfVariants [][][]float64

	var walk func(level int, preArr *[][]float64)

	walk = func(level int, preArr *[][]float64) {		
		for i := level; i < len(keys); i++ {
			for j := 0; j < len(criteria[keys[i]]); j++ {

				arr := [][]float64{}
				arr = append(arr, *preArr...) 
				arr = append(arr, criteria[keys[i]][j])
				
				if (level + 1 == len(keys)) {
					listOfVariants = append(listOfVariants, arr)
				}
				if (level < i) { continue }
				
				walk(level + 1, &arr);
			}
		}
		
	}

	d := [][]float64{}
	walk(0, &d)

	var getLeave func(tree N, name string) *N

	getLeave = func(tree N, name string) *N {
		
		var leave *N
	
		var walk func(item *N)
	
		walk = func(item *N) {
			if (leave != nil) { return }
			if item.Name == name {
				leave = item
				return
			}
			
			if len(item.Children) == 0 {
				return
			}
			
			for i := 0; i < len(item.Children); i++ {
				walk(&item.Children[i])
			}
		}
	
		walk(&tree)
	
		return leave
	}

	countSkiped := 0
	
	results := []EffectivityResult{}

	for _, v := range listOfVariants {
		sumR := 0.0
		sumE := 0.0	
		for _, vv := range v {
			sumR += vv[0]
			sumE += vv[1]
		}

		if (sumE > EMax || sumR > RMax) {
			countSkiped++
			continue
		}

		for i := 0; i < len(keys); i++ {
			leave := getLeave(clonedTree, keys[i])
			leave.Val = v[i][0]
		}

		calcNoTree(&clonedTree)

		results = append(results, EffectivityResult{
			VAL: clonedTree.Val,
			SUMR: sumR,
			SUME: sumE,
			VARIANT: v,
		})
	}

	sort.Slice(results, func(i, j int) bool {
		return results[i].VAL < results[j].VAL
	})

	elapsed := time.Since(start)
	
	fmt.Println(countSkiped, elapsed)

	return results[0]
}

// Output:
// {
//   "name": "TC",
//   "cond": 2,
//   "val": 0.039779484,
//   "children": [
//     {
//       "name": "A",
//       "cond": 2,
//       "val": 0.03969997,
//       "children": [
//         {
//           "name": "A1",
//           "cond": 0,
//           "val": 0.03,
//           "children": []
//         },
//         {
//           "name": "A2",
//           "cond": 0,
//           "val": 0.01,
//           "children": []
//         }
//       ]
//     },
//     {
//       "name": "T1",
//       "cond": 1,
//       "val": 0.000082800005,
//       "children": [
//         {
//           "name": "B",
//           "cond": 1,
//           "val": 0.0011999999,
//           "children": [
//             {
//               "name": "B1",
//               "cond": 0,
//               "val": 0.03,
//               "children": []
//             },
//             {
//               "name": "B2",
//               "cond": 0,
//               "val": 0.04,
//               "children": []
//             }
//           ]
//         },
//         {
//           "name": "C",
//           "cond": 2,
//           "val": 0.069000006,
//           "children": [
//             {
//               "name": "C1",
//               "cond": 0,
//               "val": 0.02,
//               "children": []
//             },
//             {
//               "name": "C2",
//               "cond": 0,
//               "val": 0.05,
//               "children": []
//             }
//           ]
//         }
//       ]
//     }
//   ]
// }

// Calc effectivity
// 0 {0.00030999872446513255 0.00086 4.24e+06 [[0.0003 400000] [1e-05 3e+06] [0.0005 40000] [5e-05 800000]]}