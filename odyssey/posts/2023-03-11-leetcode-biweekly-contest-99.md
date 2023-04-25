---
layout: Post
title: LeetCode Biweekly Contest 99
subtitle: Review and summary
author: Lemorage
date: 2023-03-11
useHeaderImage: true
headerImage: /img/in-post/leetcode/biweekly.png
headerMask: rgb(128, 8, 27, .5)
permalinkPattern: /post/:year/:month/:day/:slug/
tags:
  - LeetCode
  - Algorithms
---

## Q1 Split With Minimum Sum

### Solution 1

#### Intuition

Brute-force.

#### Approach

We can try all the possible ways of splitting the number into two parts, which can be represented by the 2^n^ combinations of binary number from 0 to (2^n^ - 1).

#### Complexity

 n is the number of digits.

- Time complexity: $O(2^n * n^2 \log n)$
- Space complexity: $O(n)$
    
#### Code

```cpp
int splitNum(int num) {
    string s = to_string(num);
    int n = s.size();
    sort(s.begin(), s.end());

    int res = 2e9;
    for (int i = 0; i < 1 << n; ++i)
    {
        int a = 0, b = 0;
        for (int j = 0; j < n; ++j)
        if (i >> j & 1)
            a = a * 10 + (s[j] - '0');
    	else
            b = b * 10 + (s[j] - '0');
        res = min(res, a + b);
    } 
    return res;
}
```

### Solution 2

#### Intuition

Greedy.

#### Approach

We can sort all digits, and split the digits into two substrings by taking every other digit.

#### Complexity

 n is the number of digits.

- Time complexity: $O(n \log n)$
- Space complexity: $O(n)$

#### Code

```cpp
int splitNum(int num) {
    string s = to_string(num);
    sort(s.begin(), s.end());

    string a, b;
    for (int i = 0; i < s.size(); i += 2)
    {
        a += s[i];
        b += s[i+1];
    }
    return stoi(a) + stoi(b);
}
```



## Q2 Count Total Number of Colored Cells

### Solution 1

#### Intuition

Brute-force.

#### Approach

We can observe the pattern following the iteration... 
- for n = 1, res = 1
- for n = 2, res = 1 + 4
- for n = 3, res = 1 + 4 + 8
- for n = 4, res = 1 + 4 + 8 + 12
- ......

#### Complexity

- Time complexity: $O(n)$
- Space complexity: $O(1)$
    
#### Code

```cpp
long long coloredCells(int n) {
    long long res = 1;
    for (int i = 1; i < n; ++i)
        res += 4 * i;
    return res;
}
```

### Solution 2

#### Intuition

Mathematical derivation.

#### Approach

Take n = 3 as an example:
- We can cut the original graph into a large square with a smaller square.
![img1](https://assets.leetcode.com/users/images/16c708d1-14d2-4959-9b44-83f186c79c8e_1677945863.7645595.png)
- Or we can also cut the original graph into two same rectangles with a single cell.
![img2](https://assets.leetcode.com/users/images/fb1d9b7a-78e0-4237-907a-ba7f57177373_1677947477.5902114.png)

::: warning NB
Images originated from [@lee215](https://leetcode.com/lee215). Credit goes to the original author. Please find more information at [here](https://leetcode.com/problems/count-total-number-of-colored-cells/solutions/3256196/java-c-python-cut-and-combine-o-1/?orderBy=most_votes)
:::

#### Complexity

- Time complexity: $O(1)$
- Space complexity: $O(1)$

#### Code

```cpp
long long coloredCells(int n) {
    return 1ll * n * n + 1ll * (n - 1) * (n - 1);
}

long long coloredCells_2(int n) {
    return 2ll * n * (n - 1) + 1;
}
```



## Q3 Count Ways to Group Overlapping Ranges

### Solution 1

#### Intuition

Merge intervals.

#### Approach

According to the problem statement, if there are `n` ranges in total, our result is 2^n^ % (1e9 + 7).
So the key point here is to find how many non-overlapping intervals are there.
- Sort the given ranges by the left endpoint.
- Initialize `left`, `right` to hold `INT_MIN`, which will hold the endpoint value of the previous range afterward.
- There are only two cases when we iterate through all the ranges:
    - If the left endpoint of the current range is greater than `left`, then we find a new non-overlapping range!
    - Otherwise, we just extend the right endpoint of the current range if needed.


#### Complexity

- Time complexity: $O(n \log n)$
- Space complexity: $O(n)$
    
#### Code

```cpp
int countWays(vector<vector<int>>& ranges) {
    sort(ranges.begin(), ranges.end());
    int ans = 1, mod = 1e9 + 7;
    function<int(int, int)> merge = [&](int l, int r)
    {
        for (auto seg : ranges)
        {
            if (r < seg[0])
            {
                if (l != -1) ans = ans * 2 % mod;
                l = seg[0], r = seg[1];
            } else r = max(r, seg[1]);
        }
        return ans * 2 % mod;
    };
    return merge(-1, -1);
}

int countWays_2(vector<vector<int>>& ranges) {
    sort(ranges.begin(), ranges.end());
    int r = -1, ans = 1, mod = 1e9 + 7;
    for (auto seg : ranges)
    {
        if (r < seg[0]) ans = ans * 2 % mod;
        r = max(r, seg[1]);
    }
    return ans;
}
```



## Q4 Count Number of Possible Root Nodes (TBD...)