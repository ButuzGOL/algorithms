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

### Moduli number
> A number system with moduli is deﬁned by a vector of k moduli, [m1,m2, ···,mk].  
The moduli must be pairwise co-prime, which means that, for any pair of moduli, the only common factor is 1.  
In such a system each number n is represented by a string "-x1--x2-- ... --xk-" of its residues, one for each modulus. The product m1 ... mk must be greater than the given number n which is to be converted in the moduli number system.  
For example, if we use the system [2, 3, 5] the number n = 11 is represented by "-1--2--1-",  
the number n = 23 by "-1--2--3-". If we use the system [8, 7, 5, 3] the number n = 187 becomes "-3--5--2--1-".  
You will be given a number n (n >= 0) and a system S = [m1,m2, ···,mk] and you will return a string "-x1--x2-- ...--xk-" representing the number n in the system S.  
If the moduli are not pairwise co-prime or if the product m1 ... mk is not greater than n, return "Not applicable".  
Examples:  
fromNb2Str(11 [2,3,5]) -> "-1--2--1-"  
fromNb2Str(6, [2, 3, 4]) -> "Not applicable", since 2 and 4 are not coprime  
fromNb2Str(7, [2, 3]) -> "Not applicable" since 2 * 3 < 7

```javascript
function fromNb2Str(n,sys) {
  if (sys.reduce(function(p,c) { return p * c; }) < n) return 'Not applicable';

  var min = Math.min.apply(null, sys);
  for(var i = 2, k; i <= min; i++) {
    k = 0;
    for(var j = 0; j < sys.length; j++) {
      if (sys[j] % i === 0) {
        k++;
      } 
    }  
    if (k > 1) return 'Not applicable';
  }

  return '-' + sys.map(function(i) { return n % i; }).join('--') + '-';
}

fromNb2Str(187, [8, 7, 5, 3]);

// -3--5--2--1-
```

### Backwards prime
> Backwards Read Primes are primes that when read backwards in base 10 (from right to left) are a different prime. 
(This rules out primes which are palindromes.)  
Example:
13 17 31 37 71 73 are Backwards Read Primes

```javascript
function backwardsPrime(start, stop) {
  var primes = [];
  var backwardsPrimes = [];
  
  function isPrime(num) {
    for(var i = 2; i <= Math.sqrt(num); i++) {
      if (num % i === 0) return false; 
    }
    return true;
  }
  
  function reverse(str) {
    return Number(str.toString().split('').reverse().join(''));
  }
  
  for(var i = start; i <= stop; i++) {
    if (isPrime(i)) primes.push(i);
  }
  
  for(var i = 0; i < primes.length; i++) {
    if (reverse(primes[i]) !== primes[i] && isPrime(reverse(primes[i]))) {
      backwardsPrimes.push(primes[i]);
    }
  }
  
  return backwardsPrimes; 
}

backwardsPrime(2, 100);
// [13, 17, 31, 37, 71, 73, 79, 97] 

backwardsPrime(9900, 10000);
// [9923, 9931, 9941, 9967]
```

### Heavy ball
> There are 8 balls numbered from 0 to 7. Seven of them have the same weight. One is heavier. Your task is to find it's number.  
So where's the catch, you may ask. Well - the scales is very old. You can use it only 3 TIMES before the scale breaks.

```javascript
function findBall() {
  var left = [0, 1, 2, 3];
  var right = [4, 5, 6, 7];
  
  if (scales.getWeight(left, right) === -1) {
    right = [left[2], left[3]];
    left = [left[0], left[1]];
  } else {
    left = [right[0], right[1]];
    right = [right[2], right[3]];    
  }
  
  if (scales.getWeight(left, right) === -1) {
    right = [left[1]];
    left = [left[0]];
  } else {
    left = [right[0]];
    right = [right[1]];
  }
  
  if (scales.getWeight(left, right) === -1) return left[0];
  return right[0];
}

findBall({
  getWeight: function(left, right) {
    var heavier = 2;
    if (left.indexOf(heavier) !== -1) return -1;
    if (right.indexOf(heavier) !== -1) return 1;
    return 0;
  }
});

// 2
```

### Not visible cubes
> Imagine there's a big cube consisting of n^3 small cubes. Calculate, how many small cubes are not visible from outside. 
For example, if we have a cube which has 4 cubes in a row, then the function should return 8, because there are 8 cubes inside our cube (2 cubes in each dimension)

```javascript
function notVisibleCubes(n) {
  if (n === 1) return 0;
  return n * n * n - (n * n * 2 + ((n - 2) * n * 2) + ((n - 2) * (n - 2) * 2));
}

notVisibleCubes(3);
// 1
```

### Excel sheet column numbers
> That given a column title as it appears in an Excel sheet, returns its corresponding column number.  
All column titles will be uppercase.

```javascript
function titleToNumber(title) {
  var dict = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';

  var res = 0;
  for(var i = 0; i < title.length - 1; i++) res += Math.pow(dict.length, i + 1);
  
  var index = 0;
  for(var letter of title) {
    res += dict.indexOf(letter) * Math.pow(dict.length, title.length - ++index); 
  }

  return res + 1;
}

titleToNumber('AA');
// 27
```

### Row sum odd numbers
> Given the triangle of consecutive odd numbers:
             1 |
          3     5 |
       7     9    11 |
   13    15    17    19 |
21    23    25    27    29 |
...
Calculate the row sums of this triangle from the row index.

```javascript
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
```

### Green glass door
> Step through my green glass door.  
You can take the moon, but not the sun.  
You can take your slippers, but not your sandals.  
You can go through yelling, but not shouting.  
You can't run through fast, but you can run with speed.  
You can take a sheet, but not your blanket.  
You can wear your glasses, but not your contacts.  
Have you figured it out? Good! Then write a program that can figure it out as well.

```javascript
function stepThroughWith(s) {
  var last = s[0];
  for(var i = 1; i < s.length; i++) {
    if (last === s[i]) return true; 
    last = s[i];
  }
  
  return false;
}

stepThroughWith('moon');
// true
```

## License

MIT © [ButuzGOL](https://butuzgol.github.io)
