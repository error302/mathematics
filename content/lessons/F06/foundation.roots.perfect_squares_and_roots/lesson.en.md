# Square Roots, Radicals, and Exact Values

## 1. Geometric and Algebraic Meaning of Square Roots

If a square has side length $s$, its area $A$ is:
$$A = s^2$$

Conversely, if we know the area $A$, the side length is given by the **principal square root**:
$$s = \sqrt{A}$$

Because side lengths are non-negative geometric quantities, the radical symbol $\sqrt{x}$ always denotes the **non-negative** root:
$$\sqrt{25} = 5 \quad (\text{not } -5)$$
While both $5$ and $-5$ solve the algebraic equation $x^2 = 25$, the function $\sqrt{x}$ is defined uniquely as the positive root.

## 2. Rational Radicals and Multiplication

For non-negative numbers $a, b \ge 0$:
- **Product Property**:
  $$\sqrt{a \cdot b} = \sqrt{a} \cdot \sqrt{b}$$
- **Quotient Property** ($b > 0$):
  $$\sqrt{\frac{a}{b}} = \frac{\sqrt{a}}{\sqrt{b}}$$

For example:
$$\sqrt{\frac{49}{64}} = \frac{\sqrt{49}}{\sqrt{64}} = \frac{7}{8}$$

## 3. The Additive Fallacy: $\sqrt{a + b} \neq \sqrt{a} + \sqrt{b}$

One of the most frequent algebraic traps is distributing the square root over addition.
Consider $a = 9$ and $b = 16$:
- Left hand side:
  $$\sqrt{9 + 16} = \sqrt{25} = 5$$
- Right hand side:
  $$\sqrt{9} + \sqrt{16} = 3 + 4 = 7$$

Since $5 \neq 7$, **radicals never distribute over addition or subtraction**. Always simplify the radicand (the quantity inside) before taking the root!
