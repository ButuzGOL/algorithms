function getDigit(num, pos) {
  let val = Math.floor(Math.abs(num) / Math.pow(10, pos)) % 10;
  return val;
}

getDigit(5, 1); // 5

function radixSort(arr) {
  let map = {};
  let k = 0;
  let next = true;
  while (next) {
    next = false;
    for (let i = 0; i < arr.length; i++) {
      const digit = getDigit(arr[i], k);
      if (digit !== 0) next = true;
      if (!map[digit]) map[digit] = [];
      map[digit].push(arr[i]);
    }

    const keys = Object.keys(map);
    arr = [];
    for (let i = 0; i < keys.length; i++) {
      arr = arr.concat(map[keys[i]]);
    }
    map = {};
    k++;
  }
  return arr;
}

radixSort([1, 5, 2, 3, 111, 22, 11]);
