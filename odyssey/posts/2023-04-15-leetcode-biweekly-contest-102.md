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

The key idea of this solution is to calculate the sum of values of all the cousins of a given node by subtracting the sum of values of its parent's children from the sum of values at the current level. We can use double DFS to traverse the binary tree.
> In conclusion, to calculate the sum of values of all cousins for a given node, we can assume its parent to be `p` and apply the formula S~c~ = level_sum - p_children_sum.

#### Approach

- We initialize two unordered maps to keep track of the sum of values of nodes at each depth and the sum of values of the child nodes of each parent node respectively.
- A lambda function `dfs` is defined to traverse the tree in a depth-first search manner and update the values in the hash maps.
- Another lambda function `solve` is defined to recursively traverse the tree and update the values of the nodes based on the values in the hash maps.
- The new value of each node is calculated as `level[depth] - hash[parent]`.
- The updated root node is returned.

#### Complexity

- Time complexity: O(n)
- Space complexity: O(n)
    
#### Code

```cpp
TreeNode* replaceValueInTree(TreeNode* root) {
    unordered_map<int, int> level;
    unordered_map<TreeNode*, int> hash;

    auto dfs = [&](auto&& dfs, auto node, auto parent, int depth)
    {
        if (!node) return;
        level[depth] += node->val;
        hash[parent] += node->val;
        dfs(dfs, node->left, node, depth + 1);
        dfs(dfs, node->right, node, depth + 1);
    };

    auto solve = [&](auto&& self, TreeNode*& root, auto parent, int depth)
    {
        if (!root) return;
        root->val = level[depth] - hash[parent];
        self(self, root->left, root, depth + 1);
        self(self, root->right, root, depth + 1);
    };
    
    dfs(dfs, root, nullptr, 0);
    solve(solve, root, nullptr, 0);
    return root;
}
```

### Solution 2

#### Intuition

We can also use BFS to perform level-order traversal for solving this problem.

#### Approach

- We create a queue to perform level-order traversal of the binary tree.
- While the queue is not empty:
    - Get the number of nodes at the current level and initialize a variable to track the total sum of values of child nodes.
    - Update the values of the child nodes based on the total sum and the values of their siblings.
- Repeat the above steps for each level in the tree.
- Return the root node after updating the values of all nodes in the tree.

#### Complexity

- Time complexity: O(n)
- Space complexity: O(w), `w` is the maximum width of the binary tree.
    
#### Code

```cpp
TreeNode* replaceValueInTree(TreeNode* root) {
    root->val = 0;
    queue<TreeNode*> q{{root}};

    while (!q.empty())
    {
        int n = q.size(), total = 0;
        vector<TreeNode*> level;
        for (int i = 0; i < n; ++i)
        {
            auto t = q.front(); q.pop();
            if (t->left) q.push(t->left), total += t->left->val;
            if (t->right) q.push(t->right), total += t->right->val;
            level.push_back(t);
        }

        for (auto& x : level)
        {
            int sum = total - (x->left ? x->left->val : 0) - (x->right ? x->right->val : 0);
            if (x->left) x->left->val = sum;
            if (x->right) x->right->val = sum;
        }
    }

    return root;
}
```



## Q4 Design Graph With Shortest Path Calculator

### Solution 1

#### Intuition

Plain Dijkstra.
- Instead of using a priority queue, this way of dijkstra uses a simple linear search to find the next unvisited node with the minimum distance.
- The reason for using a linear search instead of a heap is that the graph is dense. In this case, the overhead of maintaining a heap may be larger than the benefit of using it. Linear search also simplifies the implementation and can be faster for small graphs.

#### Approach

This graph is represented by **adjacency matrix**.\
The implementation starts by initializing the distance of all nodes from the starting node to infinity (represented by `0x3f3f3f3f`), except for the starting node itself which has a distance of 0. Then, it iterates through all nodes, selecting the unvisited node with the smallest distance as the next node to visit. It marks the selected node as visited and updates the distance of its unvisited neighbors if a shorter path is found. Finally, the algorithm returns the distance of the target node if there is a path or -1 if there is no path.

#### Complexity

n is the number of nodes and m is the number of edges.

The time complexity of adding an edge is O(1), and the space complexity is O(m).\
The time complexity of computing the shortest path is O(n^2^ + m). The space complexity is O(n) to store the distance array and visited set.
    
#### Code

```cpp
class Graph {
public:
    const int INF = 0x3f3f3f3f;
    vector<vector<int>> g;

    Graph(int n, vector<vector<int>>& edges) {
        g = vector<vector<int>>(n, vector<int>(n, INF));
        for (auto &e: edges)
            g[e[0]][e[1]] = e[2];
    }

    void addEdge(vector<int> e) {
        g[e[0]][e[1]] = e[2];
    }

    int shortestPath(int node1, int node2) {
        int n = g.size();
        vector<int> dist(n, INF);
        vector<bool> st(n);
        dist[node1] = 0;

        for (int i = 0; i < n; ++i)
        {
            int t = -1;
            for (int j = 0; j < n; ++j)
                if (!st[j] && (t == -1 || dist[t] > dist[j]))
                    t = j;
            st[t] = true;

            for (int j = 0; j < n; ++j)
                dist[j] = min(dist[j], dist[t] + g[t][j]);
        }

        return dist[node2] == INF ? -1 : dist[node2];
    }
};
```

### Solution 2

#### Intuition

The approach taken in Solution 2 is also based on Dijkstra's algorithm. However, it uses a priority queue to optimize the search time of finding the next unvisited node with the minimum distance.

#### Approach

This graph is represented by **adjacency list**.\
This algorithm works by maintaining a set of visited nodes and a priority queue of nodes to visit. The priority queue is sorted based on the distance from the starting node. The algorithm repeatedly extracts the node with the smallest distance from the priority queue and relaxes all its neighbors, updating their distances if a shorter path is found. This continues until the destination node is visited or the priority queue is empty.

#### Complexity

n is the number of nodes and m is the number of edges.

The time complexity of adding an edge is O(1), and the space complexity is O(m).\
The time complexity of computing the shortest path is O(m * log n). The space complexity is O(n) to store the distance array and visited set.
    
#### Code
```cpp
class Graph {
public:
    vector<vector<pair<int, int>>> adj;
    
    Graph(int n, vector<vector<int>>& edge) {
        adj.resize(n);
        for (auto& e: edge)
            adj[e[0]].emplace_back(e[1], e[2]);
    }

    void addEdge(vector<int> e) {
        adj[e[0]].emplace_back(e[1], e[2]);
    }

    int shortestPath(int node1, int node2) {
        vector<int> dist(n);
        memset(&dist[0], 0x3f, n * sizeof(int));
        vector<bool> st(n);
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<>> heap;
        heap.emplace(0, node1);
        dist[node1] = 0;

        while (!heap.empty())
        {
            auto [d, u] = heap.top(); heap.pop();
            if (st[u]) continue;
            st[u] = true;
            for (auto& edge : edges[u])
            {
                auto [v, w] = edge;
                if (dist[v] > dist[u] + w)
                {
                    dist[v] = dist[u] + w;
                    heap.emplace(dist[v], v);
                }
            }
        }

        if (dist[node2] == 0x3f3f3f3f) return -1;
        return dist[node2];
    }
};
```

### Solution 3

#### Intuition

Shortest Path Faster Algorithm (SPFA).
- SPFA is an improvement over Dijkstra's algorithm that uses a queue to optimize the search time of finding the next unvisited node with the minimum distance.
- It is similar to Dijkstra's algorithm, but instead of always selecting the node with the smallest distance, SPFA maintains a queue of nodes to visit, and it only adds a node to the queue if its distance is updated.

#### Approach

This graph is represented by **forward star edge list**.\
The implementation starts by initializing the distance of all nodes from the starting node to infinity (represented by `0x3f3f3f3f`), except for the starting node itself which has a distance of 0. Then, it uses a queue to maintain the nodes to visit, initially adding the starting node to the queue. It repeatedly dequeues a node from the queue, relaxes its neighbors by updating their distances if a shorter path is found, and enqueues them if their distances are updated. This process continues until the queue is empty, which means all nodes have been visited. Finally, the algorithm returns the distance of the target node if there is a path or -1 if there is no path.

#### Complexity

n is the number of nodes and m is the number of edges.

The time complexity of adding an edge is O(1), and the space complexity is O(m).\
The time complexity of computing the shortest path is O(n * m), but it can be optimized to O(k * m) on average, where k is the average number of edges per node. The space complexity is O(n) to store the distance array and visited set, and O(n) for the queue.
    
#### Code
```cpp
class Graph {
public:
    static const int N = 100, M = 10000;
    int head[N], e[M], ne[M], w[M], idx;
    int dist[N];
    bool st[N];
    
    Graph(int n, vector<vector<int>>& edges) {
        memset(head, -1, sizeof head);
        for (auto& e: edges)
            addEdge(e);
    }

    void addEdge(vector<int> edge) {
        int a = edge[0], b = edge[1], c = edge[2];
        e[idx] = b, w[idx] = c, ne[idx] = head[a], head[a] = idx++;
    }

    int shortestPath(int node1, int node2) {
        memset(dist, 0x3f, sizeof dist);
        memset(st, false, sizeof st);
        queue<int> q;
        q.push(node1);
        dist[node1] = 0;    
        st[node1] = true;

        while (!q.empty())
        {
            int t = q.front();
            q.pop();

            st[t] = false;

            for (int i = head[t]; ~i; i = ne[i])
            {
                int j = e[i];
                if (dist[j] > dist[t] + w[i])
                {
                    dist[j] = dist[t] + w[i];
                    if (!st[j]) q.push(j), st[j] = true;
                }
            }
        }

        if (dist[node2] == 0x3f3f3f3f) return -1;
        return dist[node2];
    }
};
```

### Solution 4

#### Intuition

Floyd-Warshall algorithm. The algorithm uses dynamic programming to find the shortest path between all pairs of nodes in a weighted graph.

#### Approach

This graph is represented by **adjacency matrix**.\
The graph is represented as an adjacency matrix in the form of a two-dimensional vector `d` of size `n x n`. The constructor of the Graph class initializes the distance between each node to `0x3f3f3f3f` (which is a large value used to represent infinity). Then, the edges are added to the d matrix. The addEdge function updates the distance between nodes `a` and `b` with weight `w`. If the current distance is greater than the new distance, the shortest path between all pairs of nodes is recomputed using the Floyd algorithm. The shortestPath function returns the shortest path between node1 and node2.

#### Complexity

n is the number of nodes and m is the number of edges.

The time complexity of adding an edge is O(n^2^), and the space complexity is O(m).\
The time complexity of computing the shortest path is O(n^3^). The space complexity is O(n^2^) to store the distance matrix.
    
#### Code
```cpp
class Graph {
public:
    vector<vector<int>> d;
    Graph(int n, vector<vector<int>>& edges) {
        d = vector(n, vector<int>(n, 0x3f3f3f3f));
        for (int i = 0; i < n; i++)
            d[i][i] = 0;
        for (auto& e : edges)
            d[e[0]][e[1]] = e[2];
        floyd(n);
    }
    
    void addEdge(vector<int> edge) {
        int a = edge[0], b = edge[1], w = edge[2];
        if (d[a][b] > w)
            for (int i = 0; i < d.size(); i++)
                for (int j = 0; j < d.size(); j++)
                    d[i][j] = min(d[i][j], d[i][a] + w + d[b][j]);
    }

    int shortestPath(int node1, int node2) {
        return d[node1][node2] == 0x3f3f3f3f ? -1 : d[node1][node2];
    }
    
private:
    void floyd(int n)
    {
        for (int k = 0; k < n; k++)
            for (int i = 0; i < n; i++)
                for (int j = 0; j < n; j++)
                    d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
    }
};
```