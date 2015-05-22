function isPangram(string){
  string = string.toLowerCase();
  return 'abcdefghijklmnopqrstuvwxyz'.split('').every(function(letter) {
    if (string.indexOf(letter) !== -1) return true;
  });
}

// console.log(isPangram('The quick brown fox jumps over the lazy dog.'));
//
// true
