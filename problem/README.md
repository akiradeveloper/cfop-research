# Problem (Algorithm)

- Time Limit: 1day
- Memory Limit: 8GB

## Statement

There are N nodes and nodes are connected by E edges.
Edges are colored and variation of colors is C.

Your task is to choose K nodes so that the graph 
consisted by the choosen nodes contains all edge colors.

What is the minimum K and the nodes to choose?

Please send me a code or description of the algorithm in PR.

## Constraints

- N < 100000
- E <= N(N-1)/2
- C <= 21

## Sample Input

N=4, C=3

```
4 3
2
1 2
3 4
1
2 3
1
2 4
```

Answer should be

```
2
3
4
```