# Polynomials, Factoring, Vieta's Formulas, and the Remainder Theorem

## 1. Factoring Quadratics and Vieta's Formulas

Suppose a quadratic polynomial with leading coefficient $A \neq 0$ factors over $\mathbb{C}$ into roots $r_1$ and $r_2$:

$$Ax^2 + Bx + C = A(x - r_1)(x - r_2) = Ax^2 - A(r_1 + r_2)x + A r_1 r_2$$

Matching coefficients of like powers of $x$ gives **Vieta's Formulas**:

$$r_1 + r_2 = -\frac{B}{A}, \qquad r_1 r_2 = \frac{C}{A}$$

When $A = 1$ (a **monic** quadratic $x^2 - sx + p = 0$), the coefficient of $x$ is the negative sum of the roots ($-(r_1 + r_2)$) and the constant term is their product ($r_1 r_2$).

## 2. The Polynomial Remainder Theorem

When any polynomial $P(x)$ is divided by a linear divisor $(x - r)$, polynomial division produces a quotient polynomial $Q(x)$ and a constant remainder $R$:

$$P(x) = (x - r)Q(x) + R$$

Because this identity holds for all $x$, substituting $x = r$ makes the first term vanish:

$$P(r) = (r - r)Q(r) + R = 0 + R = R$$

- **Remainder Theorem**: The remainder when $P(x)$ is divided by $(x - r)$ is $P(r)$.
- **Factor Theorem**: $(x - r)$ is a factor of $P(x)$ if and only if $P(r) = 0$.
