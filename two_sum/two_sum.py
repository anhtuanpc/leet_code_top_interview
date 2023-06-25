class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hash_num = {}

        for i, num in enumerate(nums):
            substract = target - num
            if substract in hash_num:
                return [i, hash_num[substract]]
            hash_num[num] = i
        return []