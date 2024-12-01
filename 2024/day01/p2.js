const fs = require('node:fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n');

let left = [];
let right = [];

for (let i = 0; i < input.length; i++) {
  let [l, r] = input[i].split('   ');
  if (!l || !r) {
    continue;
  }
  l = parseInt(l);
  r = parseInt(r);
  left.push(l);
  right.push(r);
}

let result = 0;

for (let i = 0; i < left.length; i++) {
  const l = left[i];
  
  const times = right.filter(r => r === l).length;

  console.log(l, times);

  result += l * times;
}

console.log(result);
