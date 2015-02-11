var array = [ 5, 4, 1, 6, 8, 10, 25 ];

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

// console.log(sort(array));
//
// [ 1, 4, 5, 6, 8, 10, 25 ]
