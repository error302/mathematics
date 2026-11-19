# Completing the Square, the Quadratic Formula, and the Discriminant

## Why Why-Before-How Matters for Quadratics
Every quadratic equation $ax^2 + bx + c = 0$ ($a \neq 0$) can be solved by a single idea: **symmetry around the vertex**.

Divide through by $a$:
$$x^2 + \frac{b}{a}x + \frac{c}{a} = 0$$
Add $\left(\frac{b}{2a}\right)^2$ to complete the square on the $x$ terms:
$$\left(x + \frac{b}{2a}\right)^2 = \frac{b^2 - 4ac}{4a^2}$$

## The Discriminant $\Delta = b^2 - 4ac$
The numerator on the right-hand side is the **discriminant**:
$$\Delta = b^2 - 4ac$$
Taking square roots gives the **Quadratic Formula**:
$$x = \frac{-b \pm \sqrt{\Delta}}{2a} = -\frac{b}{2a} \pm \frac{\sqrt{\Delta}}{2a}$$

- **$\Delta > 0$**: Two distinct real roots, symmetric at distance $\frac{\sqrt{\Delta}}{2|a|}$ on either side of the axis $x = -\frac{b}{2a}$. When $a, b, c \in \mathbb{Q}$ and $\Delta = k^2$ is a perfect square, both roots are exact rational numbers in $\mathbb{Q}$.
- **$\Delta = 0$**: One repeated real root $x = -\frac{b}{2a}$. Geometrically, the vertex of the parabola touches the $x$-axis tangentially. Setting $\Delta = 0$ yields the exact parameter condition:
  $$b^2 - 4ac = 0 \implies c = \frac{b^2}{4a}$$
- **$\Delta < 0$**: No real roots (the parabola does not intersect the real $x$-axis).

## Guarding Against Sign Traps
1. **Always parenthesize $b$ before squaring:** If $b = -5$, then $b^2 = (-5)^2 = +25$, never $-25$.
2. **Track the sign of $-4ac$:** When $a > 0$ and $c < 0$, the product $4ac$ is negative, so $-4ac$ is **positive** and increases $\Delta$.
3. **Divide by $2a$, not $2$:** When $a \neq 1$, the denominator of the quadratic formula is $2a$.
