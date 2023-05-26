function sameFrequency(a, b) {
  return (
    (a + "").split("").sort().join("") === (b + "").split("").sort().join("")
  );
}

sameFrequency(182, 281); // true
sameFrequency(34, 14); // false
sameFrequency(3589578, 5879385); // true
sameFrequency(22, 222); // false

function areThereDuplicates(...args) {
  const b = [];
  for (let i = 0; i < args.length; i++) {
    if (b.includes(args[i])) {
      return true;
    } else {
      b.push(args[i]);
    }
  }

  return false;
}

areThereDuplicates(1, 2, 3); // false
areThereDuplicates(1, 2, 2); // true
areThereDuplicates("a", "b", "c", "a"); // true

function averagePair(arr, val) {
  for (let i = 0; i < arr.length; i++) {
    for (let j = 0; j < arr.length; j++) {
      if (arr[i] + arr[j] / 2 === val) {
        return true;
      }
    }
  }

  return false;
}

averagePair([1, 2, 3], 2.5); // true
averagePair([1, 3, 3, 5, 6, 7, 10, 12, 19], 8); // true
averagePair([-1, 0, 3, 4, 5, 6], 4.1); // false
averagePair([], 4); // false

function isSubsequence(left, right) {
  const leftArr = left.replace(" ", "").split("");
  let rightArr = right.replace(" ", "").split("");
  let inx = -1;
  let count = 0;
  for (let i = 0; i < leftArr.length; i++) {
    if (rightArr.includes(leftArr[i])) {
      inx = rightArr.indexOf(leftArr[i]);
      rightArr = rightArr.slice(inx + 1);
      count++;
    }
  }

  return count === leftArr.length;
}

isSubsequence("hello", "hello world"); // true
isSubsequence("sing", "sting"); // true
isSubsequence("abc", "abracadabra"); // true
isSubsequence("abc", "acb"); // false (order matters)

function maxSubarraySum(arr, val) {
  if (arr.length < val) return null;
  let maxSum;

  for (let i = 0; i <= arr.length - val; i++) {
    let sum = 0;
    for (let j = 0; j < val; j++) {
      sum += arr[i + j];
    }
    if (!maxSum || sum > maxSum) {
      maxSum = sum;
    }
  }

  return maxSum;
}

maxSubarraySum([100, 200, 300, 400], 2); // 700
maxSubarraySum([1, 4, 2, 10, 23, 3, 1, 0, 20], 4); // 39
maxSubarraySum([-3, 4, 0, -2, 6, -1], 2); // 5
maxSubarraySum([3, -2, 7, -4, 1, -1, 4, -2, 1], 2); // 5
maxSubarraySum([2, 3], 3); // null

function minSubArrayLen(arr, val) {
  let minSubArrayLen = 0;
  for (let i = 0; i < arr.length; i++) {
    let sum = 0;
    for (let j = i; j < arr.length; j++) {
      sum += arr[j];
      if (sum >= val) {
        if (!minSubArrayLen || minSubArrayLen > j - i + 1) {
          minSubArrayLen = j - i + 1;
        }
        break;
      }
    }
  }

  return minSubArrayLen;
}

minSubArrayLen([2, 3, 1, 2, 4, 3], 7); // 2 -> because [4,3] is the smallest subarray
minSubArrayLen([2, 1, 6, 5, 4], 9); // 2 -> because [5,4] is the smallest subarray
minSubArrayLen([3, 1, 7, 11, 2, 9, 8, 21, 62, 33, 19], 52); // 1 -> because [62] is greater than 52
minSubArrayLen([1, 4, 16, 22, 5, 7, 8, 9, 10], 39); // 3
minSubArrayLen([1, 4, 16, 22, 5, 7, 8, 9, 10], 55); // 5
minSubArrayLen([4, 3, 3, 8, 1, 2, 3], 11); // 2
minSubArrayLen([1, 4, 16, 22, 5, 7, 8, 9, 10], 95); // 0

function findLongestSubstring(str) {
  const strA = str.split("");
  if (!strA.length) return 0;

  let maxLen = 0;
  for (let i = 0; i < strA.length; i++) {
    const subStr = [];
    for (let j = i; j < strA.length; j++) {
      if (subStr.includes(strA[j])) break;
      subStr.push(strA[j]);
    }
    if (maxLen < subStr.length) maxLen = subStr.length;
  }

  return maxLen;
}

findLongestSubstring(""); // 0
findLongestSubstring("rithmschool"); // 7
findLongestSubstring("thisisawesome"); // 6
findLongestSubstring("thecatinthehat"); // 7
findLongestSubstring("bbbbbb"); // 1
findLongestSubstring("longestsubstring"); // 8
findLongestSubstring("thisishowwedoit"); // 6

function fib(n) {
  let val = [0, 1];
  let i0 = 0;
  let i1 = 1;
  for (let i = 2; i < n; i++) {
    val.push(val[i - 2] + val[i - 1]);
    let a = i0 + i1;
    i0 = i1;
    i1 = a;
  }
  return i0 + i1;
}

function fibR(n, memo = []) {
  if (memo[n] !== undefined) return memo[n];
  if (n <= 2) return 1;
  let res = fibR(n - 1, memo) + fibR(n - 2, memo);
  memo[n] = res;
  return res;
}

console.log(fibR(5));
