function t(n) {
  for(var i = 1; i <= n; i++) {
    console.log(Array((n - i) + 1).join(' ') + Array((i + i - 1) + 1).join('*'));
  }
}

// t(4)
//    *
//   ***
//  *****
// *******
