# Function Composition and Invertibility

## 1. Chaining Mappings: Function Composition

Given two functions $g: A \to B$ and $f: B \to C$, their **composition** $f \circ g: A \to C$ is defined by feeding the output of $g$ directly into $f$:

$$(f \circ g)(x) = f(g(x))$$

- **Inside-Out Order**: Because $g$ is applied directly to $x$, you must evaluate the inner expression $u = g(x)$ **first**, and then evaluate $f(u)$ **second**.
- **Non-Commutativity**: In general, $f(g(x)) \neq g(f(x))$. For example, if $f(x) = 2x + 3$ and $g(x) = x^2$, then at $x = 4$:
  $$f(g(4)) = f(16) = 35, \quad \text{whereas} \quad g(f(4)) = g(11) = 121$$

## 2. One-to-One Functions and the Inverse Mapping

A function $f: A \to B$ is **invertible** if and only if it is a **bijection** (every output $y \in B$ comes from **exactly one** input $x \in A$).

Its inverse function $f^{-1}: B \to A$ is defined by the equivalence:

$$f^{-1}(y) = x \iff f(x) = y$$

For any invertible affine function $f(x) = ax + b$ (with $a \neq 0$):
1. Set $ax + b = y$.
2. Subtract $b$: $ax = y - b$.
3. Divide by $a$:
   $$f^{-1}(y) = \frac{y - b}{a}$$

> **Notation Warning:** In function notation, $f^{-1}(y)$ means the **inverse mapping** that reverses $f$, **never** the reciprocal $\frac{1}{f(y)}$.
