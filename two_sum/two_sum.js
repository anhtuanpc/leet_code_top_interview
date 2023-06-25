/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
  const hashTable = new Map();

  for (let i = 0; i < nums.length; i++) {
    const num = nums[i];
    const substractionWeWant = target - num;

    if (hashTable.has(substractionWeWant)) {
      return [i, hashTable.get(substractionWeWant)];
    } else {
      hashTable.set(num, i);
    }
  }
};
