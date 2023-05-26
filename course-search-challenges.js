function binarySearch(arr, val) {
  let mInx = Math.floor(arr.length / 2);

  let from = 0;
  let to = arr.length;
  while (true) {
    if (mInx < 0) return -1;
    if (arr[mInx] === val) return mInx;
    if (mInx === arr.length - 1) return -1;
    if (arr[mInx] > val) {
      to = mInx;
      mInx = from + Math.floor((to - from) / 2);
    } else {
      from = mInx;
      mInx = from + Math.floor((to - from) / 2);
    }
  }
}

binarySearch([1, 2, 3, 4, 5], 2); // 1
binarySearch([1, 2, 3, 4, 5], 3); // 2
binarySearch([1, 2, 3, 4, 5], 5); // 4
binarySearch([1, 2, 3, 4, 5], 6); // -1
binarySearch(
  [
    5, 6, 10, 13, 14, 18, 30, 34, 35, 37, 40, 44, 64, 79, 84, 86, 95, 96, 98,
    99,
  ],
  10
); // 2
binarySearch(
  [
    5, 6, 10, 13, 14, 18, 30, 34, 35, 37, 40, 44, 64, 79, 84, 86, 95, 96, 98,
    99,
  ],
  95
); // 16
binarySearch(
  [
    5, 6, 10, 13, 14, 18, 30, 34, 35, 37, 40, 44, 64, 79, 84, 86, 95, 96, 98,
    99,
  ],
  100
); // -1
