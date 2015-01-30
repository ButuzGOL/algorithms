var field = [
  [ 'O', 'X', 'X', 'X', 'X' ],
  [ 'X', 'O', 'O', 'O', 'X' ],
  [ 'X', 'O', '#', 'O', 'X' ],
  [ 'X', '*', '*', '*', 'X' ],
  [ 'X', 'X', 'O', 'X', 'X' ],
  [ 'X', 'X', 'X', '#', '#' ],
  [ 'X', 'O', 'X', 'X', 'X' ] 
];
  
function fill(field, x, y, color, replaceColor, arr) {
  arr = arr || [];
  if (arr.indexOf(y + ', ' + x) !== -1) return;
  
  if (x < 0 || x > (field[0].length - 1) ||
      y < 0 || y > (field.length - 1)) return;
  
  replaceColor = replaceColor || field[y][x];
  
  if (field[y][x] != replaceColor && field[y][x] != color) return;

  field[y][x] = color;
  arr.push(y + ', ' + x);

  fill(field, x - 1, y - 1, color, replaceColor, arr);
  fill(field, x - 1, y, color, replaceColor, arr);
  fill(field, x - 1, y + 1, color, replaceColor, arr);
  fill(field, x, y - 1, color, replaceColor, arr);
  fill(field, x, y + 1, color, replaceColor, arr);
  fill(field, x + 1, y - 1, color, replaceColor, arr);
  fill(field, x + 1, y, color, replaceColor, arr);
  fill(field, x + 1, y + 1, color, replaceColor, arr);
}

// fill(field, 0, 0, '*');
// console.log(field);
// 
// [ [ '*', 'X', 'X', 'X', 'X' ],
//   [ 'X', '*', '*', '*', 'X' ],
//   [ 'X', '*', '#', '*', 'X' ],
//   [ 'X', '*', '*', '*', 'X' ],
//   [ 'X', 'X', '*', 'X', 'X' ],
//   [ 'X', '*', 'X', '#', '#' ],
//   [ 'X', '*', 'X', 'X', 'X' ] ]
