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

Use two pointers after sorting.
- The intuition behind this algorithm is that the maximum possible greatness is achieved when the array is partitioned into the maximum number of strictly increasing subsequences. The algorithm greedily tries to extend each subsequence as far as possible by maintaining the smallest value encountered so far (`nums[j]`) and ensuring that each new element encountered (`nums[i]`) is strictly greater than the smallest value. Whenever a new subsequence is started, the "greatness" counter is incremented.

#### Approach

- Firstly, we sort all the elements in non-descending order.
- Secondly, we use two pointers, `i` and `j`, to scan through the sorted array.
    - The `j` pointer represents the left endpoint of a subarray, and it is incremented only when the value at `j` is less than the value at `i`.
    - The `i` pointer represents the right endpoint of the subarray and is incremented at each iteration of the loop.
    - For each element `nums[i]`, if there exists an element `nums[j]` to its left such that `nums[j]` is strictly less than `nums[i]`, `j` is incremented and the "greatness" counter `res` is also incremented.

#### Complexity

- Time complexity: O(n log n)
- Space complexity: O(1)
    
#### Code

```cpp
int maximizeGreatness(vector<int>& nums) {
    int res = 0;
    sort(nums.begin(), nums.end());
    for (int i = 0, j = 0; i < nums.size(); ++i)
        if (nums[j] < nums[i]) ++j, ++res;
    return res;
}
```

### Solution 2

#### Intuition

Greedy.
- In order to maximize the "greatness" of the array, we need to arrange the elements in a way that allows us to form as many adjacent pairs as possible.
- One way to do this is to put the smallest elements at the beginning of the array and the larger elements at the end, because this guarantees that we can form as many pairs as possible. However, if we put too many of the same element at the beginning, we will eventually have to pair them up with the same element later on, which doesn't contribute to our greatness score.

#### Approach

- As we want to limit the number of duplicates we put at the beginning of the array, then one way to do this is to find the maximum count of duplicates in the array and use that as our limit.
- We can use a hashmap to count the number.

::: warning NB
This solution is inspired by the brilliant idea of [@lee215](https://leetcode.com/lee215) in this [post](https://leetcode.com/problems/maximize-greatness-of-an-array/solutions/3312061/java-c-python-easy-and-concise-o-n/). Credit goes to the original author.
:::

#### Complexity

- Time complexity: O(n)
- Space complexity: O(n)

#### Code

```cpp
int maximizeGreatness(vector<int>& nums) {
    unordered_map<int, int> hash;
    int k = 0;
    for (int x : nums)
        k = max(k, ++hash[x]);
    return nums.size() - k;
}
```



## Q3 Find Score of an Array After Marking All Elements 

### Solution 1

#### Intuition

Simulation.

#### Approach

- We can just simulate the whole process after sorting, which gives us the expected result.
    - We use an additional vector to store the index and value of each element in the input vector.
    - After that, we sort this new vector based on the values of the elements.
    - We iterate through this new vector, and for each element, check if its corresponding index in the input vector has already been visited. If it has not been visited, we add the value to our result and mark its neighbors as visited.

#### Complexity

- Time complexity: O(n log n)
- Space complexity: O(n)
    
#### Code

```cpp
long long findScore(vector<int>& nums) {
    long long res = 0;
    int i = 0, n = nums.size();
    vector<pair<int, int>> pairs;

    for (int x : nums) pairs.emplace_back(x, i++);

    sort(pairs.begin(), pairs.end());
    for (const auto& [e, i] : pairs)
    {
        if (nums[i])
        {
            res += e;
            nums[i] = 0;
            if (i - 1 >= 0) nums[i-1] = 0;
            if (i + 1 < n) nums[i+1] = 0;
        }
    }
    return res;
}
```



## Q4 Minimum Time to Repair Cars

### Solution 1

#### Intuition

Binary Search. As time increases, the number of cars that can be repaired also increases.
- We use binary search to find the minimum time required to repair all the cars because it allows us to quickly eliminate large ranges of time that we know cannot be the answer.
- We can start with a range of possible times (for example, 0 to a very large number) and then repeatedly divide the range in half, checking whether the midpoint allows us to repair enough cars to meet the requirement.
- By eliminating half of the range at each step, we can converge on the minimum time required to repair all the cars much more quickly than if we were to try all possible times in the range one by one.

#### Approach

- We set the left boundary `l` to 1 (since it cannot be 0) and the right boundary `r` to the maximum time it could possibly take to repair all cars.
- Every time we check if it can repair the given number of cars in the range of `mid` time, and update `l` or `r` subsequently.

##### Noteworthy details
1. Since `cars` could be a fairly large number, we could convert `(cars * cars)` into long long type. Or we can use `pow(cars, 2)` since it returns a double type.
```
long long r = 1LL * cars * cars * ranks[0];
long long r = pow(cars, 2) * ranks[0];
```

#### Complexity

N is the maximum rank in the input array `ranks`.

- Time complexity: O(cars * log(N))
- Space complexity: O(1)
    
#### Code

```cpp
long long repairCars(vector<int>& ranks, int cars) {
    long long l = 1, r = pow(cars, 2) * ranks[0];

    auto check = [&](long long x)
    {
        long long res = 0;
        for (int r : ranks)
            res += sqrt(x / r);
        return res;
    };

    while (l < r)
    {
        long long mid = l + r >> 1;
        if (check(mid) >= cars) r = mid;
        else l = mid + 1;
    }
    return l;
}

/**
 * ANOTHER WAY:
 * We count the frequency of all `A[i]` without scanning the whole array each time,
 * since `A[i]` <= 100. This would improve the time complexity of the algorithm.
 * Time complexity: O(100 * (log(ranks[0]) + log(cars)))
 * 
 */
long long repairCars_2(vector<int>& ranks, int cars) {
    int freq[101] = {};
    for (int x : ranks) freq[x]++;

    long long l = 1, r = ranks[0] * pow(cars, 2);
    auto check = [&](long long x)
    {
        long long res = 0;
        for (int i = 1; i < 101; ++i)
            res += freq[i] * static_cast<long long>(sqrt(x / i));
        return res;
    };

    while (l < r)
    {
        long long mid = l + r >> 1;
        if (check(mid) >= cars) r = mid;
        else l = mid + 1;
    }
    return l;
}
```