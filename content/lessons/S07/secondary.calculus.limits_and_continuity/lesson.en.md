# Limits, Removable Discontinuities & Continuity

## Intuition
A **limit** $\lim_{x \to a} f(x)$ describes the behavior of $f(x)$ for inputs $x$ near $a$, independent of what happens right at $x = a$.
- If $f(x) = \frac{(x - r)(x - s)}{x - r}$, plugging in $x = r$ yields the indeterminate form $\frac{0}{0}$. However, for every $x \ne r$, the factor $(x - r)$ is non-zero and cancels exactly, leaving $f(x) = x - s$. Thus $f(x)$ approaches $r - s$ as $x \to r$.
- If we also define $f(r) = r - s$, we plug the hole and make $f$ **continuous** at $x = r$.

## Formal Definition
1. **Cancellation at a Removable Discontinuity:** For $x \ne r$,
   $$\lim_{x \to r} \frac{(x - r)(x - s)}{x - r} = \lim_{x \to r} (x - s) = r - s$$
2. **Limits at Infinity for Equal-Degree Rational Functions:** Dividing numerator and denominator by $x^2$ shows that lower-degree terms vanish as $x \to \infty$:
   $$\lim_{x \to \infty} \frac{ax^2 + bx + c}{dx^2 + ex + f} = \lim_{x \to \infty} \frac{a + \frac{b}{x} + \frac{c}{x^2}}{d + \frac{e}{x} + \frac{f}{x^2}} = \frac{a}{d}$$
3. **Continuity at a Point:** A function $f$ is continuous at $x = a$ if and only if:
   $$\lim_{x \to a} f(x) = f(a)$$

## Worked Example
**Problem:** Find the exact rational value of $k$ that makes $f(x)$ continuous at $x = 3$, where $f(x) = \frac{x^2 - 9}{x - 3}$ for $x \ne 3$ and $f(3) = 3k + 2$.

1. **Compute the limit as $x \to 3$:**
   $$\lim_{x \to 3} \frac{x^2 - 9}{x - 3} = \lim_{x \to 3} \frac{(x - 3)(x + 3)}{x - 3} = 3 + 3 = 6$$
2. **Match $f(3)$ to the limit:**
   $$3k + 2 = 6 \implies 3k = 4 \implies k = \frac{4}{3}$$

## Common Misconceptions
- **Treating $\frac{0}{0}$ as $0$:** An indeterminate $\frac{0}{0}$ limit requires algebraic simplification (factoring and canceling) before evaluation.
- **Confusing $x \to 0$ with $x \to \infty$:** At $x = 0$, $\frac{ax^2+bx+c}{dx^2+ex+f}$ evaluates to $\frac{c}{f}$, but as $x \to \infty$ the highest-degree terms dominate and the limit is $\frac{a}{d}$.
