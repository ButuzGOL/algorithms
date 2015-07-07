/*
There are 8 balls numbered from 0 to 7. Seven of them have the same weight. One is heavier. Your task is to find it's number.
So where's the catch, you may ask. Well - the scales is very old. You can use it only 3 TIMES before the scale breaks.
*/

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
