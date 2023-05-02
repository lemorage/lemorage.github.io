---
layout: Post
title: LeetCode Biweekly Contest 103
subtitle: Review and summary
author: Lemorage
date: 2023-04-29
useHeaderImage: true
headerImage: /img/in-post/leetcode/biweekly.png
headerMask: rgb(48, 8, 112, .5)
permalinkPattern: /post/:year/:month/:day/:slug/
tags:
  - LeetCode
  - Algorithms
---

## Q1 Maximum Sum With Exactly K Elements

### Solution 1

#### Intuition

We just need to find the maximum value.

#### Approach

- Find the maximum value in the vector `nums`.
- Repeat the following `k` times:
    - Add `maxv` and the current value of `k` to `res`.
- Return `res` as the result.

#### Complexity

- Time complexity: $O(n)$
- Space complexity: $O(1)$
    
#### Code

```cpp
int maximizeSum(vector<int>& nums, int k) {
    int maxv = *max_element(nums.begin(), nums.end()), res = 0;
    while (k--) res += maxv + k;
    return res;
    /* OR we can just return (maxv * 2 + k - 1) * k / 2; */
}
```



## Q2 Find the Prefix Common Array of Two Arrays

### Solution 1

#### Intuition

We can solve this problem using prefix sum with counting the common elements we've met so far. We keep track of the elements we've seen in both arrays and increment the count for each element that appears more than once. Then, we use prefix sum to calculate the total count of common elements up to each index.

#### Approach

- Initialize an array for recording all the elements we've met so far.
- Iterate through `A` and `B` array and for each element in it:
    - Increment the count of the element in the `seen` array if we've seen it before.
    - If the count for the element is 2, increment the corresponding element in the `res` array.
    - Update the current element in the `res` array based on the previous element.
- Return the `res` array in the end.

#### Complexity

- Time complexity: $O(n)$
- Space complexity: $O(1)$
    
#### Code

```cpp
vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
    int n = A.size(), seen[51] = {};
    vector<int> res(n);
    for (int i = 0; i < n; ++i)
    {
        if (++seen[A[i]] == 2) ++res[i];
        if (++seen[B[i]] == 2) ++res[i];
        if (i) res[i] += res[i - 1];
    }
    return res;
}
```



## Q3 Maximum Number of Fish in a Grid

### Solution 1

#### Intuition

The key idea of this solution is to perform a DFS on the grid to traverse all cells and collect the fish in them. We can keep track of the cells we have visited by setting their value to 0.

#### Approach

- We define a helper function `dfs` that takes in the current cell coordinates `(x, y)` and returns the total number of fish in the reachable cells.
- The helper function performs a DFS on the grid starting from `(x, y)` and returns the total number of fish in the reachable cells.
- During the DFS, we set the value of each visited cell to 0 to mark it as visited.
- We iterate through each cell in the grid and call the `dfs` function on each cell that has a positive value.
- We keep track of the maximum number of fish caught in a variable `ans` and return it.

#### Complexity

n is the number of rows and m is the number of columns in the grid.

- Time complexity: $O(nm)$
- Space complexity: $O(nm)$, for the call stack during the DFS.
    
#### Code

```cpp
int findMaxFish(vector<vector<int>>& grid) {
    int n = grid.size(), m = grid[0].size();
    int dir[5] = {0, 1, 0, -1, 0};

    auto dfs = [&](auto&& dfs, int x, int y) -> int
    {
        int fish = grid[x][y];
        grid[x][y] = 0;
        for (int i = 0; i < 4; ++i)
        {
            int a = x + dir[i], b = y + dir[i+1];
            if (a >= 0 && b >= 0 && a < n && b < m && grid[a][b])
                fish += dfs(dfs, a, b);
        }
        return fish;
    };

    int ans = 0;
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < m; ++j)
            if (grid[i][j]) ans = max(ans, dfs(dfs, i, j));
    return ans;
}
```



## Q4 Make Array Empty

### Solution 1

#### Intuition

Fenwick Tree.

#### Approach



#### Complexity


    
#### Code

```cpp
class Solution {
public:
    typedef struct fenwick_tree
    {
        vector<int> tree;

        fenwick_tree(int n) : tree(n) {}

        int lowbit(int x) { return x & -x; }

        void add(int x, int c) { for (int i = x; i < tree.size(); i += lowbit(i)) tree[i] += c; }

        int sum(int x)
        {
            int res = 0;
            for (int i = x; i; i -= lowbit(i)) res += tree[i];
            return res;
        }

        int query(int left, int right) { return sum(right) - sum(left - 1); }
    }BIT;

    long long countOperationsToEmptyArray(vector<int> &nums) {
        int n = nums.size(), id[n];
        iota(id, id + n, 0);
        sort(id, id + n, [&](int i, int j) {
            return nums[i] < nums[j];
        });

        long long ans = n;
        fenwick_tree t(n + 1);
        int pre = 1;
        for (int k = 0; k < n; ++k) {
            int i = id[k] + 1;
            if (i >= pre)
                ans += i - pre - t.query(pre, i);
            else
                ans += n - pre + i - k + t.query(i, pre - 1);
            t.add(i, 1);
            pre = i;
        }
        return ans;
    }
};
```

### Solution 2

#### Intuition

Count rotations.

#### Approach



#### Complexity


    
#### Code
```cpp
long long countOperationsToEmptyArray(vector<int>& nums) {
    int n = nums.size(), p = 0;
    long long res = n;
    unordered_map<int, int> hash;
    for (int i = 0; i < nums.size(); ++i)
        hash[nums[i]] = i;
    
    sort(nums.begin(), nums.end());
    for (int i = 0; i < nums.size(); p = hash[nums[i++]])
        if (p > hash[nums[i]]) res += n - i;
    return res;
}
```