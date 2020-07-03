## Steps Into MMR Proof


- **relay: 5**
- **game id: 5**
- **last confirmed on chain when game(5) started: 1**
- R5: mmr root when node 5 comes in, R4: mmr root when node 4 comes in.
- The sequence of mmr proof is from level low to level hight,  and from left to right

---

1. **round 0, target 5**
```
             R5
           /   \
L2        3     6
         / \   / \
L1      1   2 4   5

        proof for 1 -> 5: [2, 6]  ** the proof is from L1 to L2
            -
           / \
L2        -   6
         / \
L1      1*  2
```

2. **round 1, sample 4**
```
          R5
         / \
L2      3   -
           / \
L1        4*   5
        proof 4 -> 5: [5, 3]   ** the proof is from L1 to L2
```

3. **round 2, sample 3**
```
            R4
           / \
L2        3   \
         / \   \
L1      1   2*  4

        proof 2 -> 4: [1, 4]   ** the proof is from L1 to L2, and from Left to Right
```
