---
layout: Post
title: LeetCode Biweekly Contest 104
subtitle: Review and summary
author: Lemorage
date: 2023-05-13
useHeaderImage: true
headerImage: /img/in-post/leetcode/biweekly.png
headerMask: rgb(48, 8, 112, .5)
permalinkPattern: /post/:year/:month/:day/:slug/
tags:
  - LeetCode
  - Algorithms
---

## Q1 Number of Senior Citizens

### Solution 1

#### Intuition

To solve this problem, we can iterate over each string in the `details` array and extract the age information from the string. If the extracted age is strictly greater than 60, we increment a counter. Finally, we return the value of the counter, which represents the number of passengers who are strictly more than 60 years old.

#### Approach

- Initialize a variable `res` to keep track of the count of seniors.
- Iterate over each string in the details array.
- Extract the age substring from the string using `substr()`. This substring represents the age of the person.
- If the extracted age is strictly greater than "60", increment the res counter.
- Finally, return the value of `res`.

#### Complexity

- Time complexity: $O(n)$
- Space complexity: $O(1)$
    
#### Code

```cpp
int countSeniors(vector<string>& details) {
    int res = 0;
    for (auto d : details)
        if (d.substr(11, 2) > "60")
            ++res;
    return res;
}
```



## Q2 Sum in a Matrix

### Solution 1

#### Intuition

The problem requires finding the maximum number in each row of the given matrix, removing it, and adding it to the score. However, in terms of efficiency, we don't actually need to physically remove the elements. We can achieve the same result by sorting the elements in each row in descending order.

#### Approach

- Initialize the score `res` as 0.
- Iterate over each column `j` in the matrix.
- For each column, find the maximum number `x` in each row.
    - If it's the first column `(j = 0)`, sort the row in descending order to ensure we select the maximum number.
    - Otherwise, we can assume that the row is already sorted in descending order, so we don't need to sort it again.
- Add `x` to the score `res`.
- Repeat and finally return the final score `res`.

#### Complexity

n is the number of rows and m is the number of columns

- Time complexity: $O(n * m \log(m))$
- Space complexity: $O(1)$

#### Code

```cpp
int matrixSum(vector<vector<int>>& nums) {
    int res = 0;
    
    for (int j = 0; j < nums[0].size(); ++j)
    {
        int x = 0;
        for (int i = 0; i < nums.size(); ++i)
        {
            if (!j) sort(nums[i].rbegin(), nums[i].rend());
            x = max(x, nums[i][j]);
        }
        res += x;
    }
    return res;
}
```



## Q3 Maximum OR

### Solution 1

#### Intuition

The goal is to maximize the value obtained after applying the operation of doubling an element in the array. To achieve this, we want to left shift only the same number `k` times.

#### Approach

To determine the maximum value, we need to consider the effect of doubling an element. Let's define two arrays:

- `suffix[i]` represents the product of the elements `nums[i+1] * nums[i+2] * ... * nums[n-1]`.
- `prefix[i]` represents the product of the elements `nums[0] * nums[1] * ... * nums[i-1]`.
- The result of doubling `nums[i]` can be calculated as **`prefix[i] | (nums[i] << k) | suffix[i]`**. This formula takes into account the left side, the doubled value, and the right side of `nums[i]`.
- We can iterate through the array `nums` and calculate the maximum result by comparing it with the previous maximum. To keep track of the left side, we use a variable `pre` that accumulates the bitwise OR of the elements encountered so far.

#### Complexity

- Time complexity: $O(n)$, where n is the length of the input array `nums`.
- Space complexity: $O(n)$ for the auxiliary array `suf`
    
#### Code

```cpp
long long maximumOr(vector<int>& nums, int k) {
    int n = nums.size();
    vector<int> suf(n + 1);
    long long res = 0;

    for (int i = n - 1; i; --i)
        suf[i] = suf[i + 1] | nums[i];

    for (int i = 0, pre = 0; i < n; ++i)
    {
        res = max(res, pre | 1LL * nums[i] << k | suf[i + 1]);
        pre |= nums[i];
    }
    return res;
}
```



## Q4 Power of Heroes(TBD...)