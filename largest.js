function largest(n, xs) {
  return xs.slice(0).sort(function(a, b) { return b - a }).splice(0, n).reverse();
}

// console.log(largest(2,[10,9,8,7,6,5,4,3,2,1]));
//
// [9, 10]
