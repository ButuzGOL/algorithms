function merge(arr1, arr2) {
  let newArr = [];
  let i1 = 0;
  let i2 = 0;
  while (newArr.length !== arr1.length * 2) {
    if (arr1[i1] > arr2[i2] || arr1[i1] === undefined) {
      newArr.push(arr2[i2]);
      i2++;
    } else if (arr1[i1] < arr2[i2] || arr2[i2] === undefined) {
      newArr.push(arr1[i1]);
      i1++;
    } else {
      newArr.push(arr1[i1]);
      newArr.push(arr2[i2]);
      i1++;
      i2++;
    }
  }

  return newArr;
}

merge([1, 2, 5, 6], [3, 4, 7, 8]);

function mergeSort(arr) {
  const result = [[]];
  for (let i = 0; i < arr.length; i++) {
    result[0].push([arr[i]]);
  }

  for (let i = 0; i < arr.length / 2 - 1; i++) {
    let col = [];
    for (let j = 0; j < result[i].length; j += 2) {
      col.push(merge(result[i][j], result[i][j + 1]));
    }
    result.push(col);
  }

  return result.pop();
}

mergeSort([8, 3, 5, 4, 7, 6, 1, 2]);
