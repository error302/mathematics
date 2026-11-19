# Functions as Mappings: Domain, Piecewise Rules, and Rate of Change

## 1. The Mapping Contract: Domain, Codomain, and Range

A **function** $f: D \to C$ consists of three essential components:
1. A **domain** $D$ (the set of permissible inputs $x$),
2. A **codomain** $C$ (the target space of potential outputs),
3. A **deterministic assignment rule** $x \mapsto f(x)$ such that every $x \in D$ is mapped to **exactly one** output $f(x) \in C$.

The **range** (or **image**) of $f$ is the subset of actual outputs attained:

$$\text{Range}(f) = f(D) = \{f(x) : x \in D\} \subseteq C$$

- **Forward Evaluation**: Given an input $x_0 \in D$, substitute $x_0$ into the rule to compute the image $y_0 = f(x_0)$.
- **Pre-image Problem**: Given a target output $y_0 \in C$, solve the equation $f(x) = y_0$ to find all inputs $x \in D$ that map to $y_0$.

## 2. Piecewise-Defined Functions

A piecewise function partitions its domain into disjoint intervals, applying a tailored algebraic rule on each interval:

$$f(x) = \begin{cases} ax + b & \text{if } x < c \\ dx^2 + e & \text{if } x \ge c \end{cases}$$

To evaluate $f(x_0)$:
1. **Test the domain inequality first**: determine whether $x_0 < c$ or $x_0 \ge c$.
2. **Evaluate only the active branch**: never substitute $x_0$ into both formulas. Pay special attention to the boundary point $x_0 = c$, which belongs to the branch with the inclusive inequality ($\ge$ or $\le$).

## 3. Average Rate of Change (The Difference Quotient)

For any function $f$ on an interval $[x_1, x_2]$ with $x_1 \neq x_2$, the **average rate of change** is the ratio of the change in output to the change in input:

$$\frac{\Delta y}{\Delta x} = \frac{f(x_2) - f(x_1)}{x_2 - x_1}$$

Geometrically, this ratio is the exact slope of the **secant line** passing through the points $(x_1, f(x_1))$ and $(x_2, f(x_2))$ on the graph of $f$.
