// Fault Tree Analysis
// https://www.youtube.com/watch?v=Eq8-m6Faobo

package main

import (
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
	Val      float32 `json:"val"`
	Children []N     `json:"children"`
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

		var val float32 = 1.0
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
