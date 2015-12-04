/*
Given the triangle of consecutive odd numbers:

             1
          3     5
       7     9    11
   13    15    17    19
21    23    25    27    29
...

Calculate the row sums of this triangle from the row index.
*/

function rowSumOddNumbers(n) {
  var res = 0;
  var i = 0;
  var items = 0;
  var lvl = 1;

  while(lvl - 1 !== n) {
    i++;
    if (i % 2 !== 0) {
      items++;

      if (lvl === n) res += i;
      if (items === lvl) {
        lvl++;
        items = 0;
      }
    }

  };
  
  return res;
}

rowSumOddNumbers(2);
// 8
