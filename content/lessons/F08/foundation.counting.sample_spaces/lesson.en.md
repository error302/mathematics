# Sample Spaces, Permutations, and Combinations

## 1. The Fundamental Multiplication Principle

When a multi-stage procedure consists of $k$ sequential steps where:
- Step 1 can be performed in $n_1$ ways,
- Step 2 can be performed in $n_2$ ways (regardless of the choice in Step 1),
- $\dots$
- Step $k$ can be performed in $n_k$ ways,

the total number of distinct composite outcomes in the sample space is the product:

$$N = n_1 \times n_2 \times \dots \times n_k$$

In a decision tree, every outcome of Step 1 branches into $n_2$ possibilities at Step 2, creating an $n_1 \times n_2$ grid of leaves.

## 2. Ordered Permutations Without Replacement

Suppose we select $k$ items in a specific order from $n$ distinct items without replacement:
- 1st position: $n$ choices
- 2nd position: $n - 1$ remaining choices
- $k$-th position: $n - k + 1$ remaining choices

$$P(n, k) = n(n - 1)(n - 2)\cdots(n - k + 1) = \frac{n!}{(n - k)!}$$

## 3. Unordered Combinations

When **order does not matter** (such as choosing a committee or a subset of size $k$), all $k!$ internal rearrangements of the same $k$ chosen elements represent the **exact same subset**.

Therefore, we divide the ordered count $P(n, k)$ by $k!$:

$$\binom{n}{k} = C(n, k) = \frac{P(n, k)}{k!} = \frac{n!}{k!(n - k)!}$$

Notice the symmetry: choosing $k$ items to include automatically chooses $n - k$ items to exclude, so $\binom{n}{k} = \binom{n}{n - k}$.
