/*
Backwards Read Primes are primes that when read backwards in base 10 (from right to left) are a different prime. 
(This rules out primes which are palindromes.)
Example: 
13 17 31 37 71 73 are Backwards Read Primes
*/
function backwardsPrime(start, stop) {
  var primes = [];
  var backwardsPrimes = [];
  
  function isPrime(num) {
    for(var i = 2; i <= Math.sqrt(num); i++) {
      if (num % i === 0) return false; 
    }
    return true;
  }
  
  function reverse(str) {
    return Number(str.toString().split('').reverse().join(''));
  }
  
  for(var i = start; i <= stop; i++) {
    if (isPrime(i)) primes.push(i);
  }
  
  for(var i = 0; i < primes.length; i++) {
    if (reverse(primes[i]) !== primes[i] && isPrime(reverse(primes[i]))) {
      backwardsPrimes.push(primes[i]);
    }
  }
  
  return backwardsPrimes; 
}

backwardsPrime(2, 100);
// [13, 17, 31, 37, 71, 73, 79, 97] 
backwardsPrime(9900, 10000);
// [9923, 9931, 9941, 9967]
