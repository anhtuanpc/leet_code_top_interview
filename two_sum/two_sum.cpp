#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
    public: 
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> hashNum;

        for(int i = 0; i < nums.size(); i++) {
            int substract = target - nums[i];
            if(hashNum.count(substract)) {
                return { hashNum[substract], i };
            }

            hashNum[nums[i]] = i;
        }

        return {};
    }
};