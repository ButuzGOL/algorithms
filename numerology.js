function numerology(date){
  var num = date.getDate() + (date.getMonth() + 1) + date.getFullYear();
  while(num > 10) {
    num = num.toString().split('').map(Number).reduce(function(pv, cv) { return pv + cv; }, 0);
  }
  return num;
}

// console.log(numerology(new Date('09/07/1989')));
//
// 7
