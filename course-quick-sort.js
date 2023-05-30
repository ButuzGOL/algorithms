function quickSort(arr) {
  let map = {};

  let initInx;
  let inx;
  let k = 0;
  while (Object.keys(map).length != arr.length) {
    if (map[k]) {
      k++;
      continue;
    }
    let val = arr[k];
    inx = k;
    initInx = k;
    for (let j = initInx; j < arr.length; j++) {
      if (val > arr[j]) {
        inx++;
        let tmp = arr[inx];
        arr[inx] = arr[j];
        arr[j] = tmp;
      }
    }
    let tmp = arr[inx];
    arr[inx] = arr[initInx];
    arr[initInx] = tmp;
    map[inx] = true;
    k = 0;
  }
  return arr;
}
quickSort([5, 2, 1, 8, 4, 7, 6, 3]);
