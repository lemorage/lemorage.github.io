---
layout: Post
title: LeetCode Biweekly Contest 100
subtitle: Review and summary
author: Lemorage
date: 2023-03-18
useHeaderImage: true
headerImage: /img/in-post/leetcode/biweekly.png
headerMask: rgb(128, 8, 27, .5)
permalinkPattern: /post/:year/:month/:day/:slug/
tags:
  - LeetCode
  - Algorithms
---

## Q1 Distribute Money to Maximum Children

### Solution 1

#### Intuition

Brute-force.

#### Approach

- If the money is less than the children, then return -1.
- Otherwise, we distribute each children by 1 dollar to satisfy the second condition.
    - We then use a while loop to check how many children can receive 7 dollars each, subtracting 7 dollars from the remaining money until there is not enough money left or no more children left to distribute the money to.
    - After the loop, we check the edge cases.
        - If there is any remaining money left after all the children have received their share, it subtracts one from our result.
        - If there is only one child left to receive money, and there is exactly three units of money left to distribute. In this case, we also need to subtract one from our result.

#### Complexity

- Time complexity: O(money / 7)
- Space complexity: O(1)
    
#### Code

```cpp
int distMoney(int money, int children) {
    if (money < children) return -1;

    int res = 0;
    money -= children;
    while (money >= 7 && children)
    {
        money -= 7;
        children--;
        res++;
    }

    if (res)
    {
        if (children == 0 && money) --res;
        if (children == 1 && money == 3) --res;
    }
    return res;
}
```



## Q2 Maximize Greatness of an Array 

### Solution 1

#### Intuition



#### Approach



#### Complexity

- Time complexity: O(n)
- Space complexity: O(1)
    
#### Code

```cpp

```

### Solution 2

#### Intuition



#### Approach



#### Complexity

- Time complexity: O(1)
- Space complexity: O(1)

#### Code

```cpp

```



## Q3 Find Score of an Array After Marking All Elements 

### Solution 1

#### Intuition



#### Approach



#### Complexity

- Time complexity: O(n)
- Space complexity: O(n)
    
#### Code

```cpp

```



## Q4 Minimum Time to Repair Cars