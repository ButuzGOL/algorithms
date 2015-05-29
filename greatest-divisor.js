function greatestDivisor(x, y) {
  return y == 0 ? x : greatestDivisor(y, x % y)
}

// console.log(greatestDivisor(12, 18));
//
// 6
