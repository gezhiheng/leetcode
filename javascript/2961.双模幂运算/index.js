/**
 * @param {number[][]} variables
 * @param {number} target
 * @return {number[]}
 */
var getGoodIndices = function(variables, target) {
  let result = []
  for (let i = 0; i < variables.length; i++) {
    if (isGoodArray(variables[i], target)) {
      result.push(i)
    }
  }
  return result
};

function isGoodArray(array, target) {
  return Math.pow((Math.pow(array[0], array[1]) % 10), array[2]) % array[3] === target
}

console.log(getGoodIndices([37,1,12,52], 1))

console.log(Math.pow(7, 12) % 52)