func twoSum(nums []int, target int) []int {
    hashNum := make(map[int]int)

    for i, num := range nums {
        substract := target - num
        if index, found := hashNum[substract]; found {
            return []int{i, index}
        }
        hashNum[num] = i
    }

    return nil
}