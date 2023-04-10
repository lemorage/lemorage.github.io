---
layout: Post
title: LeetCode Biweekly Contest 101
subtitle: Review and summary
author: Lemorage
date: 2023-04-01
useHeaderImage: true
headerImage: /img/in-post/leetcode/biweekly.png
headerMask: rgb(128, 8, 27, .5)
permalinkPattern: /post/:year/:month/:day/:slug/
tags:
  - LeetCode
  - Algorithms
---

## Q1 Form Smallest Number From Two Digit Arrays

### Solution 1

#### Intuition

Bitmask. We can use bitmask to mark the position of each digit.
Suppose `i=3` and `mask1` initially has all bits set to zero. With `mask1 |= 1 << i`, we would get `mask1` equal to `00001000`, as the 3rd bit (counting from the rightmost bit starting at 0) is set to 1.

#### Approach

- `1 << i` is equivalent to `pow(2, i)`, which sets the i-th bit in the mask to 1.
- Similarly, `n >> k & 1` checks if the k-th bit in n is set to 1 or 0 by shifting n to the right by k bits and then performing a bitwise AND with 1.

#### Complexity

- Time complexity: O(n)
- Space complexity: O(1)
    
#### Code

```cpp
int minNumber(vector<int>& nums1, vector<int>& nums2) {
    int mask1 = 0, mask2 = 0;
    for (int i : nums1) mask1 |= 1 << i;
    for (int j : nums2) mask2 |= 1 << j;

    int k = mask1 & mask2;
    if (k)
    {
        int bit = 0;
        while (!(k >> bit & 1)) ++bit;
        return bit;
    }

    int bit1 = 0, bit2 = 0;
    while (!(mask1 >> bit1 & 1)) ++bit1;
    while (!(mask2 >> bit2 & 1)) ++bit2;
    return min(bit1 * 10 + bit2, bit2 * 10 + bit1);
}
```

### Solution 2

#### Intuition

We can start by counting the frequency of each digit in both arrays. 
- If there is any digit that appears in both arrays, then we can just return that digit as the answer. 
- Otherwise, we can pick the smallest digit from each array and form a two-digit number.

#### Approach

- We first initialize an array of size 10 to keep track of the frequency of each digit. 
- Then, we iterate over the digits from 1 to 9 and check if a digit has a frequency of 2.
- If we haven't found a common digit yet, we pick the smallest digit from each array. We form two numbers by concatenating the smallest digits of each array. We compare the two numbers formed and return the smallest one.

#### Complexity

- Time complexity: O(n)
- Space complexity: O(1)
    
#### Code

```cpp
int minNumber(vector<int>& nums1, vector<int>& nums2) {
    int freq[10] = {};
    for (int i : nums1) freq[i]++;
    for (int j : nums2) freq[j]++;

    for (int k = 1; k < 10; ++k)
        if (freq[k] == 2) return k;

    int x = *min_element(nums1.begin(), nums1.end()), y = *min_element(nums2.begin(), nums2.end());
    return min(x * 10 + y, y * 10 + x);
}
```



## Q2 Find the Substring With Maximum Cost

### Solution 1

#### Intuition

Kadane's algorithm.
::: details
Kadane's algorithm is based on the following observation: for each position in the array, the maximum subarray ending at that position is either the element at that position itself or the sum of the maximum subarray ending at the previous position and the element at the current position.

The key observation of Kadane's algorithm can be understood by breaking down the problem of finding the maximum subarray sum into smaller subproblems.

Consider an array A with n elements. Let M[i] denote the maximum subarray sum ending at position i, and let S[i] denote the sum of the subarray ending at position i. We want to find the maximum value of M[i] for all i from 0 to n-1.

Now consider the subproblem of finding the maximum subarray sum ending at position i. There are two possibilities:

The maximum subarray sum ending at position i is just the element A[i] itself.
The maximum subarray sum ending at position i is the sum of the maximum subarray ending at position i-1 and the element A[i].
To see why this observation makes sense, consider the case where the maximum subarray sum ending at position i is the sum of the maximum subarray ending at position i-1 and the element A[i]. This means that the maximum subarray ending at position i-1 must have a positive sum, otherwise we would just take the element A[i] as the maximum subarray ending at position i.

In other words, if the maximum subarray ending at position i-1 has a negative sum, then including A[i] in the subarray will not increase the sum, and we are better off just taking A[i] as the maximum subarray ending at position i. On the other hand, if the maximum subarray ending at position i-1 has a positive sum, then including A[i] in the subarray will increase the sum, and we should take the sum of the maximum subarray ending at position i-1 and A[i] as the maximum subarray ending at position i.

Therefore, we can solve the problem of finding the maximum subarray sum by iterating through the array once and keeping track of the maximum subarray sum ending at each position. The final answer is the maximum value of the maximum subarray sums. This is the essence of Kadane's algorithm.
```
def max_subarray_sum(arr):
    max_sum = current_sum = arr[0]
    for num in arr[1:]:
        current_sum = max(num, current_sum + num)
        max_sum = max(max_sum, current_sum)
    return max_sum
```
:::

#### Approach

- Create an unordered map to store the values of each character for quick lookups.
- Initialize two integer variables: cur and res to keep track of the maximum value.
- For each character `c` in the given string `s`, update the value of cur to be the maximum of either 0 or the sum of `cur` and the value of `c`. Also, update the value of `res` to be the maximum of `cur` and `res`.
- Return the final value of res.

#### Complexity

- Time complexity: O(n)
- Space complexity: O(1)
    
#### Code

```cpp
int maximumCostSubstring(string s, string chars, vector<int>& vals) {
    unordered_map<char, int> hash;
    for (int i = 0; i < vals.size(); ++i) hash[chars[i]] = vals[i];

    auto value = [&](int x) {return hash.count(x) ? hash[x] : (x - 'a' + 1); };

    int cur = 0, res = 0;
    for (char c : s)
        cur = max(0, cur + value(c)), res = max(cur, res);
    return res;
}
```



## Q3 Make K-Subarray Sums Equal

### Solution 1

#### Intuition

Find median of each GCD-defined subarray.
- Because the array `arr` is circular. Therefore, the element at index `i + k` is actually the same as the element at index `i` if we consider the circular nature of the array. Similarly, the element at index `i + n` is also the same as the element at index `i`.
> Every k size subarray sums equal => a[i] + a[i+1] + ... + a[i+k-1] = a[i+1] + a[i+2] + ... + a[i+k]
- So if we have found that `a[i] = a[i + k]` and `a[i] = a[i + n]`, then it follows that `a[i + k] = a[i + n]`, which is why we can find a cycle of length `gcd(n, k)`. 
> According to <u>Bézout's Theorem</u>, arr has both a cycle n and a cycle k, so it must have a cycle gcd(n, k)
- The key idea of the code is to calculate the median value of each of these subarrays and use it to determine the number of operations needed to make the sum of each subarray equal.

#### Approach

- We find the greatest common divisor (GCD) between `k` and the length of the array, and use this as the new value of `k`. This ensures that there are no partially overlapping subarrays and all subarrays of length `k` can be considered.
- We iterates from `0` to `k-1` to consider all possible starting points of subarrays of length `k`.
- In order to find the minimum number of operations needed to make every k-sized subarray sum equal, we calculate the mid-value of each subarray by sorting them and taking the middle value.
- Iterate over every kth element of the circular array and calculate the sum of absolute differences between each element and its corresponding mid-value. This gives the number of operations needed for each subarray.
- In summary, the code calculates the greatest common divisor of `k` and the length of the array `arr`, which ensures that all subarrays of length `k` can be considered. Then, for each subarray of length `k`, the code sorts the elements, finds the median value, and calculates the absolute difference between each element and the median value. Finally, it sums the absolute differences to determine the minimum number of operations required to make the subarrays of length `k` in the array arr have equal sums.


#### Complexity

- Time complexity: O(n log k)
- Space complexity: O(n)
    
#### Code

```cpp
long long makeSubKSumEqual(vector<int>& arr, int k) {
    int n = arr.size();
    k = gcd(n, k);

    long long res = 0;
    for (int i = 0; i < k; ++i)
    {
        vector<int> v;
        for (int j = i; j < n; j += k)
            v.push_back(arr[j]);
        sort(v.begin(), v.end());

        int mid = v[v.size() / 2];
        for (int x : v) res += abs(mid - x);
    }
    return res;
}
```
```python
def makeSubKSumEqual(self, arr: List[int], k: int) -> int:
    k, res = gcd(k, len(arr)), 0

    for i in range(k):
        v = sorted(arr[i::k])
        mid = v[len(v) // 2]
        res += sum([abs(mid - x) for x in v])
    return res
```



## Q4 Shortest Cycle in a Graph

### Solution 1

#### Intuition

BFS
- We perform a BFS (breadth-first search) on each vertex in the graph, and for each vertex, it considers all its neighbors that haven't been visited yet. If the distance from the current vertex to a neighbor k is greater than 1 (meaning that k is not a neighbor of the current vertex's parent), then there exists a cycle of length dist[k] + dist[j] + 1, where j is the current vertex and dist[k] and dist[j] are the distances from the source vertex to k and j, respectively.
- If we consider all possible pairs of vertices in the cycle, the shortest distance between any two vertices in the cycle is guaranteed to be less than or equal to dist[k] + dist[j] + 1, because any longer path would necessarily include the cycle at least twice, and therefore be longer than the shortest cycle.
- Since the algorithm considers all vertices and all edges in the graph, it guarantees that it will find the shortest cycle in the graph, if one exists.
::: details Why it guarantee to find the shortest cycle in the graph?
Suppose we are currently at vertex j and we have a neighbor k such that dist[k] is greater than dist[j] - 1. This means that k is not a neighbor of the current vertex's parent (if it had one), since if k was a neighbor of the parent, then we would have dist[k] = dist[j] - 1.

Let d be the length of the shortest path from vertex i to vertex k passing through j. Then we have:

dist[k] = dist[j] + d

Since k is not a neighbor of j's parent, we know that the path from i to k passing through j does not go back to the parent of j. Therefore, the path from i to k passing through j must form a cycle. This cycle has length:

dist[k] + dist[j] + 1 = dist[j] + d + dist[j] + 1 = 2 * dist[j] + d + 1

Therefore, we have found a cycle of length 2 * dist[j] + d + 1. Since we are updating the result res to be the minimum cycle length we find, we can be sure that we have found the shortest cycle in the graph.
:::

#### Approach

- Create an adjacency list to represent the graph using the provided edges.
- Set the initial value for the shortest cycle to a large number, because we want to find the smallest cycle.
- For each node `i` in the graph, perform the following:
    - Create a `dist` vector with initial value of `inf` for each node, except `i`, which is set to 0.
    - Use a queue to perform a breadth-first search starting from node `i`.
    - For each node `j` visited, update its distance from node `i` in the "dist" vector and add its neighbors to the queue.
    - If a neighbor `k` has a distance already assigned in the "dist" vector, and it is not equal to the distance from `j` to `i` minus 1, then a cycle has been found, and update the shortest cycle length, `res`, if necessary.
- If the shortest cycle length, `res`, is still the initial value, return -1, indicating that no cycle was found. Otherwise, return the shortest cycle length.

#### Complexity

n is the number of vertices and m is the number of edges in the graph.

- Time complexity: O(n(m + n))
- Space complexity: O(m + n)
    
#### Code

```cpp
int findShortestCycle(int n, vector<vector<int>>& edges) {
    vector<vector<int>> adj(n);
    for (auto& e : edges)
    {
        adj[e[0]].push_back(e[1]);
        adj[e[1]].push_back(e[0]);
    }
    
    int res = inf;
    for (int i = 0; i < n; ++i)
    {
        vector<int> dist(n, inf);
        dist[i] = 0;
        queue<int> q{{i}};
        
        while (!q.empty())
        {
            int j = q.front(); q.pop();
            for (auto k : adj[j])
            {
                if (dist[k] == inf)
                {
                    dist[k] = dist[j] + 1;
                    q.push(k);
                }
                else if (dist[k] != dist[j] - 1)
                    res = min(res, dist[k] + dist[j] + 1);
            }
        }
    }
    
    return res < inf ? res : -1;
}
```