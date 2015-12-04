/*
Imagine there's a big cube consisting of n^3 small cubes. Calculate, how many small cubes are not visible from outside.
For example, if we have a cube which has 4 cubes in a row, then the function should return 8, because there are 8 cubes inside our cube (2 cubes in each dimension)
*/

function notVisibleCubes(n) {
  if (n === 1) return 0;
  return n * n * n - (n * n * 2 + ((n - 2) * n * 2) + ((n - 2) * (n - 2) * 2));
}

notVisibleCubes(3);
// 1
