var field = [
  [ 'O', 'X', 'X', 'X', 'X' ],
  [ 'X', 'O', 'O', 'O', 'X' ],
  [ 'X', 'O', '#', 'O', 'X' ],
  [ 'X', '*', '*', '*', 'X' ],
  [ 'X', 'X', 'O', 'X', 'X' ],
  [ 'X', 'X', 'X', '#', '#' ],
  [ 'X', 'O', 'X', 'X', 'X' ] 
];
  
function fill(field, x, y, color, replaceColor, setted) {
  setted = setted || [];
  
  if (setted.indexOf(x + ', ' + y) !== -1) return;
  
  if (x < 0 || x > (field[0].length - 1) ||
      y < 0 || y > (field.length - 1)) return;
  
  replaceColor = replaceColor || field[y][x];
  
  if (field[y][x] != replaceColor && field[y][x] != color) return;

  field[y][x] = color;
  
  setted.push(x + ', ' + y);

  fill(field, x - 1, y - 1, color, replaceColor, setted);
  fill(field, x - 1, y, color, replaceColor, setted);
  fill(field, x - 1, y + 1, color, replaceColor, setted);

  fill(field, x, y - 1, color, replaceColor, setted);
  fill(field, x, y + 1, color, replaceColor, setted);

  fill(field, x + 1, y - 1, color, replaceColor, setted);
  fill(field, x + 1, y, color, replaceColor, setted);
  fill(field, x + 1, y + 1, color, replaceColor, setted);
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
