# Arithmetic Progressions & Gauss Series Sums

## Intuition
An **arithmetic progression** (or arithmetic sequence) is a sequence of numbers where the difference between consecutive terms is constant. If the first term is $a_1$ and the common difference is $d$, the sequence unfolds step by step:
$$a_1, \quad a_1 + d, \quad a_1 + 2d, \quad a_1 + 3d, \quad \dots$$
Notice the index pattern: the 2nd term has $1d$, the 3rd term has $2d$, and the $n$-th term has $(n - 1)d$ because it takes $(n - 1)$ steps to walk from the 1st fencepost to the $n$-th fencepost.

## Formal Definition
A sequence $(a_n)_{n \ge 1}$ is arithmetic with common difference $d \in \mathbb{Q}$ if $a_{n+1} - a_n = d$ for all $n \ge 1$.
1. **Explicit $n$-th Term:**
   $$a_n = a_1 + (n - 1)d$$
2. **Difference from Distant Terms:** For any two indices $p \ne q$,
   $$a_q - a_p = (q - p)d \implies d = \frac{a_q - a_p}{q - p}$$
3. **Gauss's Arithmetic Series Sum:** Writing $S_n = a_1 + a_2 + \dots + a_n$ forward and backward and adding column-by-column yields $2S_n = n(a_1 + a_n)$, so:
   $$S_n = \frac{n(a_1 + a_n)}{2} = \frac{n\bigl(2a_1 + (n - 1)d\bigr)}{2}$$

## Worked Example
**Problem:** In an arithmetic sequence, $a_1 = 4$ and $d = 3$. Find $a_{10}$ and the sum $S_{10}$ of the first $10$ terms.

1. **Count steps to the 10th term:**
   $$a_{10} = a_1 + (10 - 1)d = 4 + 9(3) = 31$$
2. **Pair first and last terms:**
   $$a_1 + a_{10} = 4 + 31 = 35$$
3. **Apply Gauss's formula:**
   $$S_{10} = \frac{10(4 + 31)}{2} = 5 \times 35 = 175$$

## Common Misconceptions
- **Fencepost error ($n$ instead of $n - 1$):** Adding $n \cdot d$ to $a_1$ actually computes $a_{n+1}$.
- **Omitting division by $2$:** $n(a_1 + a_n)$ is twice the sum $2S_n$; always divide by $2$.
- **Inclusive counting on index gaps:** Between $a_4$ and $a_9$ there are $9 - 4 = 5$ steps of $d$, not $6$.
