var field = [
  [ 'O', 'X', 'X', 'X', 'X' ],
  [ 'X', 'O', 'O', 'O', 'X' ],
  [ 'X', 'O', '#', 'O', 'X' ],
  [ 'X', '*', '*', '*', 'X' ],
  [ 'X', 'X', 'O', 'X', 'X' ],
  [ 'X', 'X', 'X', '#', '#' ],
  [ 'X', 'O', 'X', 'X', 'X' ] 
];
  
function fill(field, startX, startY, color) {
  var pixelStack;
  var replaceColor = field[startY][startX];
  var setted = [];

  pixelStack = [
    [startX, startY]
  ];

  function checkPixel(x, y, color, replaceColor) {
    try {
      return (field[y][x] == color || field[y][x] == replaceColor);
    } catch(e) {
      return false;
    }
  }

  while(pixelStack.length) {
    var currentPixel = pixelStack.pop();
    var x = currentPixel[0];
    var y = currentPixel[1];

    if (setted.indexOf(x + ', ' + y) !== -1) continue;
    
    field[y][x] = color;
    setted.push(x + ', ' + y);

    if (checkPixel(x - 1, y - 1, color, replaceColor)) pixelStack.push([x - 1, y - 1]);
    if (checkPixel(x - 1, y, color, replaceColor)) pixelStack.push([x - 1, y]);
    if (checkPixel(x - 1, y + 1, color, replaceColor)) pixelStack.push([x - 1, y + 1]);

    if (checkPixel(x, y - 1, color, replaceColor)) pixelStack.push([x, y - 1]);
    if (checkPixel(x, y + 1, color, replaceColor)) pixelStack.push([x, y + 1]);
    
    if (checkPixel(x + 1, y - 1, color, replaceColor)) pixelStack.push([x + 1, y - 1]);
    if (checkPixel(x + 1, y, color, replaceColor)) pixelStack.push([x + 1, y]);
    if (checkPixel(x + 1, y + 1, color, replaceColor)) pixelStack.push([x + 1, y + 1]);
  }

}

// fill(field, 0, 0, '*');
// console.log(field);
//
// [ [ '*', 'X', 'X', 'X', 'X' ],
//   [ 'X', '*', '*', '*', 'X' ],
//   [ 'X', '*', '#', '*', 'X' ],
//   [ 'X', '*', '*', '*', 'X' ],
//   [ 'X', 'X', '*', 'X', 'X' ],
//   [ 'X', 'X', 'X', '#', '#' ],
//   [ 'X', 'O', 'X', 'X', 'X' ] ]
