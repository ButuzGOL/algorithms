function power(val, pow) {
  if (!pow) return 1;
  return val * power(val, pow - 1);
}

// power(2,0) // 1
// power(2,2) // 4
// power(2,4) // 16

function factorial(val) {
  if (val == 1) return 1;
  return val * factorial(val - 1);
}

// factorial(1) // 1
// factorial(2); // 2
// factorial(4)); // 24
// console.log(factorial(7)); // 5040

function reverse(str) {
  if (!str.length) return "";
  return str[str.length - 1] + reverse(str.slice(0, str.length - 1));
}

console.log(reverse("awesome")); // 'emosewa'

// 2 + 1
// 1 + 0
// 0 = 0
function recursiveRange(num) {
  if (num == 0) return 0;
  const val = recursiveRange(num - 1);
  return num + val;
}

console.log(recursiveRange(2));

function productOfArray(arr) {
  if (arr.length === 1) return arr[0];
  const first = arr[0];
  const second = arr[1];
  arr.shift();
  arr.shift();
  return productOfArray([first * second, ...arr]);
}

// 1. 1 * 2
// 2. [2, 3, 4]
// 2 * 3
// 3. [6, 4]
// 6 * 4
// 4. [24]

console.log(productOfArray([1, 2, 3, 4]));

function isPalindrome(val) {
  const first = val.slice(0, 1);
  const last = val.slice(val.length - 1);
  val = val.substring(1);
  val = val.substring(0, val.length - 1);
  if (first !== last) return false;
  if (val === "" || val.lenght === 1) return true;
  return isPalindrome(val);
}

isPalindrome("tacocat");

function someRecursive(arr, fn) {
  if (!arr.length) return false;
  let result = fn(arr[0]);
  arr.shift();
  if (result === true) return true;
  return someRecursive(arr, fn);
}

console.log(someRecursive([1, 2, 3], (x) => x > 1));

function flatten(arr) {
  for (let i = 0; i < arr.length; i++) {
    if (Array.isArray(arr[i])) {
      arr.splice(i, 1, ...flatten(arr[i]));
    }
  }

  return arr;
}

console.log(flatten([1, 2, 3, [4, [5]]]));
