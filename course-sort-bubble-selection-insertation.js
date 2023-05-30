function bubbleSort(arr) {
  let sorted = true;
  while (sorted) {
    sorted = false;
    for (let i = 0; i < arr.length; i++) {
      if (arr[i + 1] && arr[i] > arr[i + 1]) {
        const tmp = arr[i];
        arr[i] = arr[i + 1];
        arr[i + 1] = tmp;
        sorted = true;
      }
    }
  }

  return arr;
}

bubbleSort([1, 2, 5, 3, 6]);

function bubbleSort1(arr) {
  for (let i = 0; i < arr.length; i++) {
    for (let j = 0; j < arr.length - i; j++) {
      if (arr[j] > arr[j + 1]) {
        const tmp = arr[j];
        arr[j] = arr[j + 1];
        arr[j + 1] = tmp;
      }
    }
  }

  return arr;
}

bubbleSort1([9, 1, 2, 5, 3, 6]);

function selectionSort(arr) {
  for (let i = 0; i < arr.length; i++) {
    let minInx = i;
    for (let j = i; j < arr.length; j++) {
      if (arr[minInx] > arr[j]) {
        minInx = j;
      }
    }
    let tmp = arr[i];
    arr[i] = arr[minInx];
    arr[minInx] = tmp;
  }

  return arr;
}

selectionSort([9, 1, 2, 5, 3, 6]);

function insertationSort(arr) {
  for (let i = 1; i < arr.length; i++) {
    let curVal = arr[i];
    for (var j = i - 1; j >= 0 && arr[j] > curVal; j--) {
      arr[j + 1] = arr[j];
    }
    arr[j + 1] = curVal;
  }

  return arr;
}

insertationSort([9, 1, 2, 5, 3, 6]);
