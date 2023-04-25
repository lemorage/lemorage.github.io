---
layout: Post
title: LeetCode Weekly Contest 339
subtitle: Review and summary
author: Lemorage
date: 2023-04-02
useHeaderImage: true
headerImage: /img/in-post/leetcode/weekly.png
headerMask: rgb(48, 8, 112, .5)
permalinkPattern: /post/:year/:month/:day/:slug/
tags:
  - LeetCode
  - Algorithms
---

## Q1 Find the Longest Balanced Substring of a Binary String

### Solution 1

#### Intuition

We can use two pointers approach.
- We count the consecutive 0s firstly, then we count the consecutive 1s secondly.
- The result is the minimum of them, and we multiply it by 2.

#### Approach

- Loop through the characters of the string s using the variable `i`.
- If the current character is '1', continue to the next iteration of the loop.
- Initialize a variable `j` to `i` and continue incrementing `j` while the character at the j-th position is '0'.
- Initialize a variable `k` to `j` and continue incrementing `k` while the character at the k-th position is '1'.
- Calculate the length of the balanced substring as the minimum between the difference of `k-j` and `j-i`, and multiply it by 2. After that, we store the maximum length found so far in the variable `res`.

#### Complexity

- Time complexity: $O(n)$
- Space complexity: $O(1)$
    
#### Code

```cpp
int findTheLongestBalancedSubstring(string s) {
    int n = s.size(), res = 0;
    for (int i = 0; i < s.size(); ++i)
    {
        if (s[i] == '1') continue;
        int j = i;
        while (j < n && s[j] == '0') ++j;
        int k = j;
        while (k < n && s[k] == '1') ++k;
        res = max(res, min(k - j, j - i) * 2);
        i = k - 1;
    }
    return res;
}
```



## Q2 Convert an Array Into a 2D Array With Conditions

### Solution 1

#### Intuition

The size of the result vector will be determined by the element with the maximum frequence. So we can use sorting approach here to make all the duplicates be placed next to each other.

#### Approach

- We sort `nums` in ascending order, and group together any identical values that appear consecutively in the sorted vector. - During iteration, we use two pointers to count the frequency of an element. 
- Once we have the frequency of the current unique element, we resize the result vector res to be able to hold all the duplicates of the current element.
- Then, we iterate over the duplicates and push them in `res`.
- Finally, we return the resulting matrix `res`.

#### Complexity

- Time complexity: $O(n \log n)$
- Space complexity: $O(n)$
    
#### Code

```cpp
vector<vector<int>> findMatrix(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    vector<vector<int>> res;
    int n = nums.size();

    for (int i = 0, j; i < n; i = j)
    {
        for (j = i + 1; j < n && nums[j] == nums[i]; ++j);
        
        int cnt = j - i;
        if (cnt > res.size()) res.resize(cnt, {});

        for (int k = 0; k < cnt; ++k)
            res[k].push_back(nums[i]);
    }
    return res;
}
```

### Solution 2

#### Intuition

The size of the result vector will be determined by the element with the maximum frequence. So we can also use a hash table to store the frequency of each element.

#### Approach

- We use an `unordered_map` to count the frequency of each number in `nums`.
- Then, for each distinct number in the map, we checks if its frequency is greater than the size of the result vector.
    - If it is, the code first fills the existing rows with the number until it reaches the same frequency as the number's count, and then adds new rows for any remaining counts.
    - If the frequency is less than or equal to the number of rows in the result vector, the code fills the existing rows with the number until it runs out of counts.
- Finally, the resulting matrix is returned.

#### Complexity

- Time complexity: $O(n^2)$
- Space complexity: $O(n)$

#### Code

```cpp
vector<vector<int>> findMatrix(vector<int>& nums) {
    unordered_map<int, int> hash;
    for (int x : nums) hash[x]++;

    vector<vector<int>> res;
    for (auto [x, y] : hash)
    {
        if (y > res.size())
        {
            for (int i = 0; i < res.size(); ++i)
                res[i].push_back(x);
            res.insert(res.end(), y - res.size(), {x});
        }
        else
            for (int i = 0; i < y; ++i) res[i].push_back(x);
    }
    return res;
}
```



## Q3 Mice and Cheese

### Solution 1

#### Intuition

For any two cheese pieces, `i` and `j`, we want to determine which mouse should eat which cheese to maximize the total reward. We can achieve this by comparing the sum of rewards for the two possible pairings of the mice and cheeses: `reward1[i] + reward2[j] and reward1[j] + reward2[i]`.

To simplify this comparison, we can reformulate it as `reward1[i] - reward2[i] > reward1[j] - reward2[j]`. This is equivalent to saying that mouse `i` should eat the cheese with the higher net reward (`reward1[i] - reward2[i]`), as compared to mouse `j` eating the cheese with the higher net reward (`reward1[j] - reward2[j]`).

We can create a difference array `diff` that stores the net rewards for each mouse and cheese pair, where `diff[i] = reward1[i] - reward2[i]`. This allows us to directly compare the net rewards of different pairs of mice and cheeses by comparing the corresponding values in `diff`.

Finally, we can find the `k` largest values in `diff` using a heap or sorting and add them to the result.

#### Approach

- Initialize the result `res` to the sum of all the rewards in `reward2`.
- Subtract the rewards in `reward2` from the corresponding rewards in `reward1`.
- Create a difference array `diff` as `diff[i] = reward1[i] - reward2[i]`.
- Sort the difference array in descending order.
- Add the `k` largest elements of `diff` to `res`.
- Return `res`.

#### Complexity

- Time complexity: $O(n \log n)$, due to sorting the `diff` array.
- Space complexity: $O(1)$
    
#### Code

```cpp
int miceAndCheese(vector<int>& reward1, vector<int>& reward2, int k) {
    int n = reward1.size(), res = accumulate(reward2.begin(), reward2.end(), 0);

    for (int i = 0; i < n; ++i) reward1[i] -= reward2[i];
    sort(reward1.rbegin(), reward1.rend());

    for (int i = 0; i < k; ++i) res += reward1[i];
    return res;
}
```



## Q4 Minimum Reverse Operations (TBD...)