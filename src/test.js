function calculateSum(a, b) { return a + b; }
function generateRandom() { return Math.floor(Math.random() * 100); }
const numbers = [1, 2, 3, 4, 5].map(x => x * 2);
console.log(`The sum is: ${calculateSum(5, generateRandom())}`);
document.querySelector('#result').innerHTML = numbers.join(', ');