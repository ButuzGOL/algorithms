/*
That given a column title as it appears in an Excel sheet, returns its corresponding column number.
All column titles will be uppercase.
*/

function titleToNumber(title) {
  var dict = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';

  var res = 0;
  for(var i = 0; i < title.length - 1; i++) res += Math.pow(dict.length, i + 1);
  
  var index = 0;
  for(var letter of title) {
    res += dict.indexOf(letter) * Math.pow(dict.length, title.length - ++index); 
  }

  return res + 1;
}

titleToNumber('AA');
// 27
