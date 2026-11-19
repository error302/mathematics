# Derivatives, The Power Rule & Tangent Lines

## Intuition
The **derivative** $f'(x_0)$ is the instantaneous rate of change of $f(x)$ at $x = x_0$, geometrically equal to the slope of the line tangent to the graph $y = f(x)$ at $(x_0, f(x_0))$.

Using the Binomial Theorem on the difference quotient of $x^n$:
$$\frac{(x + h)^n - x^n}{h} = \frac{\bigl(x^n + n x^{n-1}h + \binom{n}{2}x^{n-2}h^2 + \dots\bigr) - x^n}{h} = n x^{n-1} + \binom{n}{2}x^{n-2}h + \dots$$
As $h \to 0$, every term still containing $h$ vanishes, leaving the exact **Power Rule** $\frac{d}{dx}(x^n) = n x^{n-1}$.

## Formal Definition
1. **Limit Definition of the Derivative:**
   $$f'(x) = \lim_{h \to 0} \frac{f(x + h) - f(x)}{h}$$
2. **Polynomial Differentiation:** For $f(x) = ax^3 + bx^2 + cx + d$,
   $$f'(x) = 3ax^2 + 2bx + c$$
3. **Tangent Line at $x = x_0$:** Passing through $(x_0, f(x_0))$ with slope $m = f'(x_0)$,
   $$y = f'(x_0)(x - x_0) + f(x_0) = f'(x_0)x + \underbrace{\bigl(f(x_0) - f'(x_0)x_0\bigr)}_{B}$$
4. **Critical Points (Horizontal Tangents):** Solutions to $f'(x) = 0$. For $f(x) = ax^2 - bx + c$, $f'(x) = 2ax - b = 0 \implies x = \frac{b}{2a}$.

## Worked Example
**Problem:** Find the $y$-intercept $B$ of the line tangent to $f(x) = 2x^2 + 3x + 5$ at $x_0 = 2$.

1. **Point on the curve:** $f(2) = 2(2^2) + 3(2) + 5 = 19$.
2. **Derivative and slope:** $f'(x) = 4x + 3 \implies m = f'(2) = 4(2) + 3 = 11$.
3. **Tangent line $y$-intercept:**
   $$B = f(2) - f'(2)\cdot 2 = 19 - 11(2) = -3$$

## Common Misconceptions
- **Leaving the constant $d$ in $f'(x)$:** A constant term adds vertical shift without changing slope, so $\frac{d}{dx}(d) = 0$.
- **Using $f(0)$ as the tangent line's $y$-intercept:** $f(0) = c$ is where the curve crosses the $y$-axis, whereas the tangent line at $x_0$ crosses at $B = f(x_0) - f'(x_0)x_0$.
