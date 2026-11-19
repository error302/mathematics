# Geometric Sequences & Convergent Infinite Series

## Intuition
In a **geometric progression**, each term is obtained by multiplying the preceding term by a constant non-zero **common ratio** $r$:
$$a, \quad ar, \quad ar^2, \quad ar^3, \quad \dots, \quad ar^{n-1}$$
To sum the first $n$ terms without adding them one by one, compare $S_n$ and $r S_n$:
$$\begin{aligned}
S_n &= a + ar + ar^2 + \dots + ar^{n-1} \\
r S_n &= \phantom{a + {}} ar + ar^2 + \dots + ar^{n-1} + ar^n
\end{aligned}$$
Subtracting the two rows telescopes every interior term, leaving $(r - 1)S_n = a(r^n - 1)$.

## Formal Definition
Let $a = g_1$ be the first term and $r$ be the common ratio.
1. **Explicit $n$-th Term:**
   $$g_n = a \cdot r^{n-1}$$
2. **Finite Geometric Series ($r \ne 1$):**
   $$S_n = \sum_{k=1}^{n} a r^{k-1} = \frac{a(r^n - 1)}{r - 1} = \frac{a(1 - r^n)}{1 - r}$$
3. **Convergent Infinite Geometric Series ($|r| < 1$):** Since $\lim_{n \to \infty} r^n = 0$ when $|r| < 1$,
   $$S_\infty = \lim_{n \to \infty} S_n = \frac{a}{1 - r}$$

## Worked Example
**Problem:** Evaluate the infinite geometric series $6 + 2 + \frac{2}{3} + \frac{2}{9} + \dots$ over $\mathbb{Q}$.

1. **Identify $a$ and $r$:** The first term is $a = 6$ and the common ratio is $r = \frac{2}{6} = \frac{1}{3}$, which satisfies $|r| < 1$.
2. **Compute $1 - r$:**
   $$1 - \frac{1}{3} = \frac{2}{3}$$
3. **Divide $a$ by $1 - r$:**
   $$S_\infty = \frac{6}{2/3} = 6 \times \frac{3}{2} = 9$$

## Common Misconceptions
- **Order of operations in $a \cdot r^{n-1}$:** Exponentiate $r^{n-1}$ first, then multiply by $a$; never compute $(ar)^{n-1}$.
- **Exponent confusion between $g_n$ and $S_n$:** The $n$-th term uses $r^{n-1}$, whereas the sum of the first $n$ terms uses $r^n - 1$.
- **Sign in the infinite sum denominator:** The formula is always $\frac{a}{1 - r}$, so for positive $r = \frac{p}{q}$ the denominator shrinks to $1 - \frac{p}{q} = \frac{q - p}{q}$.
