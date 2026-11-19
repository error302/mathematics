# Function Transformations, Symmetry, and Vertex Form

## 1. Coordinate Mapping of Function Transformations

Given a parent function $y = f(x)$, consider the transformed family:

$$g(x) = a\,f(x - h) + k$$

Suppose $(x_0, y_0)$ is a known point on the parent graph, so $f(x_0) = y_0$.
- To feed the same inner value $x_0$ into $f$, we must set $x - h = x_0 \implies x = x_0 + h$.
- At $x = x_0 + h$, the output becomes $g(x_0 + h) = a\,f(x_0) + k = a y_0 + k$.

Therefore, the transformation maps every point on the graph according to the exact coordinate rule:

$$(x_0, y_0) \longmapsto (x_0 + h, \; a y_0 + k)$$

## 2. Vertex Form of a Quadratic Function

Applying this transformation to the parent parabola $f(x) = x^2$ (whose vertex is at $(0, 0)$) yields **vertex form**:

$$g(x) = a(x - h)^2 + k$$

- **Vertex**: $(h, k)$, achieved where $x - h = 0 \implies x = h$.
- **Axis of Symmetry**: the vertical line $x = h$.
- **Extremum**: Because $(x - h)^2 \ge 0$ for all $x \in \mathbb{R}$, if $a > 0$ the parabola opens upward with minimum value $k$; if $a < 0$ it opens downward with maximum value $k$.
