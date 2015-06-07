function isLeapYear(year) {
  return (year % 100 !== 0 && year % 4 === 0) || year % 400 === 0;
}

// console.log(isLeapYear(1984));
//
// true
