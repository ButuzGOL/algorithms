# Algoritms
A Collection of JavaScript tasks based on algorithms, etc. with solutions

### New year tree
```javascript
function t(n) {
  for(var i = 1; i <= n; i++) {
    console.log(Array((n - i) + 1).join(' ') + Array((i + i - 1) + 1).join('*'));
  }
}

t(4)

//    *
//   ***
//  *****
// *******
```

### Bucket fill tool
[Source](https://github.com/ButuzGOL/algoritms/blob/master/bucket-fill-tool.js)  
[Source recursion](https://github.com/ButuzGOL/algoritms/blob/master/bucket-fill-tool-recursion.js)
```javascript
var field = [
  [ 'O', 'X', 'X', 'X', 'X' ],
  [ 'X', 'O', 'O', 'O', 'X' ],
  [ 'X', 'O', '#', 'O', 'X' ],
  [ 'X', '*', '*', '*', 'X' ],
  [ 'X', 'X', 'O', 'X', 'X' ],
  [ 'X', 'X', 'X', '#', '#' ],
  [ 'X', 'O', 'X', 'X', 'X' ] 
];
fill(field, 0, 0, '*');

// [ [ '*', 'X', 'X', 'X', 'X' ],
//   [ 'X', '*', '*', '*', 'X' ],
//   [ 'X', '*', '#', '*', 'X' ],
//   [ 'X', '*', '*', '*', 'X' ],
//   [ 'X', 'X', '*', 'X', 'X' ],
//   [ 'X', '*', 'X', '#', '#' ],
//   [ 'X', '*', 'X', 'X', 'X' ] ]
```

### Facebook events calendar
[Source](https://github.com/ButuzGOL/algoritms/blob/master/fb-calendar.js)
```javascript
var events = [
  { start: 0, end: 30 },
  { start: 30, end: 100 },
  { start: 80, end: 160 },
  { start: 110, end: 240 },
  { start: 110, end: 180 },
  { start: 170, end: 230 },
  { start: 170, end: 240 },
  { start: 170, end: 220 },
  { start: 190, end: 260 },
  { start: 230, end: 300 },
  { start: 240, end: 280 },
  { start: 250, end: 290 },
  { start: 300, end: 330 }
];
draw(build(events));

// 10  ||                                                                              |
// 20  ||                                                                              |
// 30  ||______________________________________________________________________________|
// 40  ||              |
// 50  ||              |
// 60  ||              |
// 70  ||              |
// 80  ||              |
// 90  ||              ||              |
// 100 ||______________||              |
// 110 |                |              |
// 120 ||              ||              ||              |
// 130 ||              ||              ||              |
// 140 ||              ||              ||              |
// 150 ||              ||              ||              |
// 160 ||              ||______________||              |
// 170 ||              |                |              |
// 180 ||______________||              ||              ||              ||              |
// 190 |                |              ||              ||              ||              |
// 200 ||              ||              ||              ||              ||              |
// 210 ||              ||              ||              ||              ||              |
// 220 ||              ||______________||              ||              ||              |
// 230 ||              |                |              ||______________||              |
// 240 ||              |                |______________||              ||______________|
// 250 ||              |                                |              ||              |
// 260 ||______________|                |              ||              ||              |
// 270 |                                |              ||              ||              |
// 280 |                                |              ||              ||______________|
// 290 |                                |______________||              |
// 300 |                                                |______________|
// 310 ||                                                                              |
// 320 ||                                                                              |
// 330 ||______________________________________________________________________________|

```

### Bubble sort
```javascript
function sort(array) {
  var swapped = true;
  var j = 0;
  var tmp;

  do {
    swapped = false;

    j++;
    for(var i = 0; i < array.length - j; i++) {
      if (array[i] > array[i + 1]) {
        tmp = array[i];
        array[i] = array[i + 1];
        array[i + 1] = tmp;

        swapped = true;
      }
    }
  } while(swapped);

  return array;
}

console.log(sort([ 5, 4, 1, 6, 8, 10, 25 ]));

// [ 1, 4, 5, 6, 8, 10, 25 ]
```

### Greatest divisor
```javascript
function greatestDivisor(x, y) {
  return y == 0 ? x : greatestDivisor(y, x % y)
}

console.log(greatestDivisor(12, 18));

// 6
```

### Is leap year
```javascript
function isLeapYear(year) {
  return (year % 100 !== 0 && year % 4 === 0) || year % 400 === 0;
}

console.log(isLeapYear(1984));

// true
```

### Is pangram
> A pangram is a sentence that contains every single letter of the alphabet at least once. For example, the sentence "The quick brown fox jumps over the lazy dog" is a pangram, because it uses the letters A-Z at least once (case is irrelevant).

```javascript
function isPangram(string){
  string = string.toLowerCase();
  return 'abcdefghijklmnopqrstuvwxyz'.split('').every(function(letter) {
    if (string.indexOf(letter) !== -1) return true;
  });
}

console.log(isPangram('The quick brown fox jumps over the lazy dog.'));

// true
```

### Largest
```javascript
function largest(n, xs) {
  return xs.slice(0).sort(function(a, b) { return b - a }).splice(0, n).reverse();
}

console.log(largest(2, [10,9,8,7,6,5,4,3,2,1]));

// [9, 10]
```

### Numerology
```javascript
function numerology(date){
  var num = date.getDate() + (date.getMonth() + 1) + date.getFullYear();
  while(num > 10) {
    num = num.toString().split('').map(Number).reduce(function(pv, cv) { return pv + cv; }, 0);
  }
  return num;
}

console.log(numerology(new Date('09/07/1989')));

// 7
```

### Phone number
```javascript
function phoneNumber(numbers) {
  return numbers.join('').replace(/(...)(...)(.*)/, '($1) $2-$3');
}

console.log(phoneNumber([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));

// (123) 456-7890
```

## License

MIT © [ButuzGOL](https://butuzgol.github.io)
