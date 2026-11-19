# Recurrence Relations & The Binomial Theorem

## Intuition
A **recurrence relation** specifies how to construct the next state of a system from its current state. Starting from a base value $a_1$, the rule $a_n = c a_{n-1} + d$ combines geometric scaling ($c$) and arithmetic translation ($d$) at every step.

Combinatorics and algebra meet in the **Binomial Theorem**. When expanding $(ax + b)^n$, we multiply $n$ identical factors $(ax + b)$. To form the $x^{n-k}$ term, we must select $b$ from exactly $k$ of the brackets and $ax$ from the other $n - k$ brackets. Because there are $\binom{n}{k}$ ways to choose those $k$ brackets, the resulting term is $\binom{n}{k}(ax)^{n-k}b^k$.

## Formal Definition
1. **Iterative Recurrence Evaluation:** Given $a_1$ and $a_n = c a_{n-1} + d$ ($n \ge 2$), each term is determined uniquely by forward substitution:
   $$a_2 = c a_1 + d, \quad a_3 = c a_2 + d, \quad a_4 = c a_3 + d, \quad \dots$$
2. **The Binomial Theorem:** For any integer $n \ge 0$,
   $$(ax + b)^n = \sum_{k=0}^{n} \binom{n}{k} (ax)^{n-k} b^k = \sum_{k=0}^{n} \binom{n}{k} a^{n-k} b^k x^{n-k}$$
   where $\binom{n}{k} = \frac{n!}{k!(n-k)!}$.

## Worked Example
**Problem:** Find the exact coefficient of $x^2$ in the expansion of $(2x + 3)^4$.

1. **Match the exponent:** With $n = 4$ and $n - k = 2$, we have $k = 2$.
2. **Write the $k = 2$ binomial term:**
   $$\binom{4}{2}(2x)^{4-2}(3)^2 = 6 \cdot (2^2 x^2) \cdot 9$$
3. **Multiply the numerical factors:**
   $$6 \times 4 \times 9 = 216$$

## Common Misconceptions
- **Treating $a_n = c a_{n-1} + d$ as purely arithmetic:** Multiply the entire previous term $a_{n-1}$ by $c$ before adding $d$ at each step.
- **Omitting $a^{n-k}$ in $(ax + b)^n$:** Remember that $(ax)^{n-k} = a^{n-k} x^{n-k}$; the coefficient $a$ is exponentiated along with $x$.
