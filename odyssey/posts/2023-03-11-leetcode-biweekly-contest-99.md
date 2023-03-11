---
layout: Post
title: LeetCode Biweekly Contest 99
subtitle: My solution to this contest...
author: Lemorage
date: 2023-03-07
useHeaderImage: true
headerImage: /img/in-post/leetcode/header.png
headerMask: rgb(5, 18, 127, .5)
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

- Time complexity: O(2^n * n * n * log n)
- Space complexity: O(n)
    
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

- Time complexity: O(n log n)
- Space complexity: O(n)

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

- Time complexity: O(n)
- Space complexity: O(1)
    
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
- Or we can also cut the original graph into two same rectangles with a single cell. 

#### Complexity

- Time complexity: O(1)
- Space complexity: O(1)

#### Code

```cpp
long long coloredCells_1(int n) {
    return 1ll * n * n + 1ll * (n - 1) * (n - 1);
}

long long coloredCells_2(int n) {
    return 2ll * n * (n - 1) + 1;
}
```


