# Exponentials, Logarithms, Log Laws, and Common-Base Equations

## Logarithms as Inverse Exponentiation
For a base $b > 0$ ($b \neq 1$) and positive argument $y > 0$, the base-$b$ logarithm $\log_b(y)$ is the exact exponent to which $b$ must be raised to produce $y$:
$$\log_b(y) = k \iff b^k = y$$

- **Positive integer powers:** Since $2^5 = 32$, $\log_2(32) = 5$.
- **Zero power:** Since $b^0 = 1$, $\log_b(1) = 0$ for every valid base $b$.
- **Reciprocal powers (negative exponents):** Since $\frac{1}{b^k} = b^{-k}$, we have $\log_b\!\left(\frac{1}{b^k}\right) = -k$. For example, $\log_3\!\left(\frac{1}{27}\right) = \log_3(3^{-3}) = -3$.

## The Laws of Logarithms
Because logarithms output exponents, the laws of exponents translate directly into laws of logarithms for $A, B > 0$:
1. **Product Law (addition of exponents $\leftrightarrow$ multiplication of arguments):**
   $$\log_b(A) + \log_b(B) = \log_b(A \cdot B)$$
2. **Quotient Law (subtraction of exponents $\leftrightarrow$ division of arguments):**
   $$\log_b(A) - \log_b(B) = \log_b\!\left(\frac{A}{B}\right)$$
3. **Power Law:**
   $$\log_b(A^m) = m \log_b(A)$$

Combined example:
$$\log_2(12) + \log_2(4) - \log_2(3) = \log_2\!\left(\frac{12 \times 4}{3}\right) = \log_2(16) = \log_2(2^4) = 4$$

## Solving Exponential Equations via a Common Base
Because $x \mapsto b^x$ is one-to-one, two powers of the same base are equal if and only if their exponents are equal. To solve
$$b^{ax + c} = (b^m)^{dx + e}$$
first apply the power-of-a-power rule $(b^m)^{dx + e} = b^{m(dx + e)} = b^{mdx + me}$ (distributing $m$ to **both** $dx$ and $e$), and then equate exponents:
$$ax + c = m d x + m e \implies (a - md)x = me - c \implies x = \frac{me - c}{a - md}$$
