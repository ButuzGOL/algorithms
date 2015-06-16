function backwardsPrime(start, stop) {
  var primes = [];
  var backwardsPrimes = [];
  for(var i = start; i < stop; i++) {
    k = true;
    for(var j = 2; j < i; j++) {
      if (i % j === 0) k = false; 
    }
     
    if (k) {
      primes.push(i);
    }
  }
  
  function reverse(str) {
    return Number(str.toString().split('').reverse().join(''));
  }
console.log(primes)
  for(var i = 0; i < primes.length; i++) {
    if (primes[i].toString().length === 1) continue;
     
    if (primes.some(function(prime) { return reverse(prime) !== prime && reverse(prime) === primes[i]; })) {
      backwardsPrimes.push(primes[i]);
    }
     
  }

  return backwardsPrimes; 
}
