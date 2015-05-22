function phoneNumber(numbers) {
  return numbers.join('').replace(/(...)(...)(.*)/, '($1) $2-$3');
}

// console.log(phoneNumber([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
//
// (123) 456-7890
