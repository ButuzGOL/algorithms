# Algoritms
A Collection of JavaScript tasks based on algorithms, etc. with solutions

### New year tree
[Source](https://github.com/ButuzGOL/algoritms/blob/master/new-year-tree.js)

```javascript
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
[Source](https://github.com/ButuzGOL/algoritms/blob/master/bubble-sort.js)

```javascript
console.log(sort([ 5, 4, 1, 6, 8, 10, 25 ]));

// [ 1, 4, 5, 6, 8, 10, 25 ]
```

### Greatest divisor
[Source](https://github.com/ButuzGOL/algorithms/blob/master/greatest-divisor.js)

```javascript
console.log(greatestDivisor(12, 18));

// 6
```

### Is leap year
[Source](https://github.com/ButuzGOL/algorithms/blob/master/is-leap-year.js)

```javascript
console.log(isLeapYear(1984));

// true
```

### Is pangram
[Source](https://github.com/ButuzGOL/algorithms/blob/master/is-panagram.js)
> A pangram is a sentence that contains every single letter of the alphabet at least once. For example, the sentence "The quick brown fox jumps over the lazy dog" is a pangram, because it uses the letters A-Z at least once (case is irrelevant).

```javascript
console.log(isPangram('The quick brown fox jumps over the lazy dog.'));

// true
```

### Largest
[Source](https://github.com/ButuzGOL/algorithms/blob/master/largest.js)

```javascript
console.log(largest(2, [10,9,8,7,6,5,4,3,2,1]));

// [9, 10]
```

### Numerology
[Source](https://github.com/ButuzGOL/algorithms/blob/master/numerology.js)

```javascript
console.log(numerology(new Date('09/07/1989')));

// 7
```

### Phone number
[Source](https://github.com/ButuzGOL/algorithms/blob/master/phone-number.js)

```javascript
console.log(phoneNumber([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));

// (123) 456-7890
```

### Phone number
[Source](https://github.com/ButuzGOL/algorithms/blob/master/phone-number.js)

```javascript
console.log(phoneNumber([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));

// (123) 456-7890
```

## License

MIT © [ButuzGOL](https://butuzgol.github.io)
