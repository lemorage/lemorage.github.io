---
layout: Post
title: LeetCode Biweekly Contest 102
subtitle: Review and summary
author: Lemorage
date: 2023-04-15
useHeaderImage: true
headerImage: /img/in-post/leetcode/biweekly.png
headerMask: rgb(48, 8, 112, .5)
permalinkPattern: /post/:year/:month/:day/:slug/
tags:
  - LeetCode
  - Algorithms
---

## Q1 Find the Width of Columns of a Grid

### Solution 1

#### Intuition

Brute-force.

#### Approach

- Iterate through each column of the grid.
- For each column, iterate through each row and calculate the length of the string representation of the number at the current row and column using the `to_string` function.
> Note: `s.size()` returns an `std::size_t`, which is an unsigned integer type. So we need to cast its type.
- Compare this length with the current maximum length stored in ans[j], and update ans[j] if necessary.

#### Complexity

- Time complexity: O(n^2)
- Space complexity: O(n)
    
#### Code

```cpp
vector<int> findColumnWidth(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    vector<int> ans(n);
    for (int j = 0; j < n; ++j)
        for (int i = 0; i < m; ++i)
            ans[j] = max((int)to_string(grid[i][j]).size(), ans[j]);
    return ans;
}
```



## Q2 Find the Score of All Prefixes of an Array

### Solution 1

#### Intuition

# Intuition
Prefix Sum with the track of current max value.
- Normally we calculate prefix sum by `prefix_sum[i] = prefix_sum[i-1] + nums[i]`.
- In this problem, we need to add an extra value `cur_max`, i.e. `prefix_sum[i] = prefix_sum[i-1] + nums[i] + cur_max`.

#### Approach

Iterate through the input array and for each element:
- Update the maximum value seen so far `maxv` by comparing the current element with `maxv`.
- Calculate the prefix score for the current element using the formula `res[i] = res[i-1] + nums[i] + maxv`.
- Store the prefix score in the result array.

#### Complexity

- Time complexity: O(n)
- Space complexity: O(n)
    
#### Code

```cpp
vector<long long> findPrefixScore(vector<int>& nums) {
    vector<long long> res(nums.size());
    long long maxv = 0;
    for (int i = 0; i < nums.size(); ++i)
    {
        maxv = max(nums[i] * 1LL, maxv);
        res[i] = res[max(0, i-1)] + nums[i] + maxv;
    }
    return res;
}
```



## Q3 Cousins in Binary Tree II

### Solution 1

#### Intuition

Double DFS.

#### Approach

- Two unordered maps `hash1` and `hash2` are defined to keep track of the sum of values of nodes at each depth and the sum of values of the child nodes of each parent node respectively.
- A lambda function `dfs` is defined to traverse the tree in a depth-first search manner and update the values in the hash maps.
- Another lambda function `solve` is defined to recursively traverse the tree and update the values of the nodes based on the values in the hash maps.
- The `dfs` function is called with the root node and `nullptr` as the parent node to update the hash maps.
- The `solve` function is called with the root node and `nullptr` as the parent node to update the values of the nodes in the tree.
- The updated root node is returned.

#### Complexity

- Time complexity: O(n)
- Space complexity: O(n)
    
#### Code

```cpp
TreeNode* replaceValueInTree(TreeNode* root) {
    unordered_map<int, int> hash1;
    unordered_map<TreeNode*, int> hash2;

    auto dfs = [&](auto&& dfs, auto node, auto parent, int depth)
    {
        if (!node) return;
        hash1[depth] += node->val;
        hash2[parent] += node->val;
        dfs(dfs, node->left, node, depth + 1);
        dfs(dfs, node->right, node, depth + 1);
    };

    auto solve = [&](auto&& self, TreeNode*& root, auto parent, int depth)
    {
        if (!root) return;
        root->val = hash1[depth] - hash2[parent];
        self(self, root->left, root, depth + 1);
        self(self, root->right, root, depth + 1);
    };
    
    dfs(dfs, root, nullptr, 0);
    solve(solve, root, nullptr, 0);
    return root;
}
```



## Q4 Design Graph With Shortest Path Calculator

### Solution 1

#### Intuition

Dijkstra.

#### Approach

Dijkstra's algorithm works by maintaining a set of visited nodes and a priority queue of nodes to visit. The priority queue is sorted based on the distance from the starting node. The algorithm repeatedly extracts the node with the smallest distance from the priority queue and relaxes all its neighbors, updating their distances if a shorter path is found. This continues until the destination node is visited or the priority queue is empty.

#### Complexity

n is the number of nodes and m is the number of edges.

The time complexity of adding an edge is O(1), and the space complexity is O(m).
The time complexity of computing the shortest path is O(n^2^ + m). The space complexity is O(n) to store the distance array and visited set.
    
#### Code

```cpp
class Graph {
public:
    vector<vector<pair<int, int>>> adj;
    Graph(int n, vector<vector<int>>& edges) {
        adj.resize(n);
        for (auto& e: edges)
            adj[e[0]].emplace_back(e[1], e[2]);
    }

    void addEdge(vector<int> e) {
        adj[e[0]].emplace_back(e[1], e[2]);
    }

    int shortestPath(int node1, int node2) {
        int n = adj.size();
        vector<int> dist(n, 0x3f3f3f3f);
        vector<bool> st(n);
        dist[node1] = 0;

        for (int i = 0; i < n; ++i) {
            int t = -1;
            for (int j = 0; j < n; ++j) {
                if (!st[j] && (t == -1 || dist[t] > dist[j])) {
                    t = j;
                }
            }
            st[t] = true;

            for (auto& neighbor : adj[t]) {
                int v = neighbor.first;
                int w = neighbor.second;
                dist[v] = min(dist[v], dist[t] + w);
            }
        }

        if (dist[node2] == 0x3f3f3f3f) {
            return -1;
        }
        return dist[node2];
    }
};
```