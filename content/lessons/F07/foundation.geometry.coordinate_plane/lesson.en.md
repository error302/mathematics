# The Cartesian Coordinate Plane and Distance

## 1. The 2D Coordinate Grid

The Cartesian coordinate plane $\mathbb{R}^2$ provides a bridge between geometry and algebra:
- The horizontal directed line is the **$x$-axis**.
- The vertical directed line is the **$y$-axis**.
- Their intersection $(0, 0)$ is the **origin**.

Any point $P$ is designated by an ordered pair $(x, y)$, where:
- $x$ represents horizontal signed displacement from the $y$-axis.
- $y$ represents vertical signed displacement from the $x$-axis.

The axes divide the plane into four quadrants:
- **Quadrant I**: $x > 0, y > 0$
- **Quadrant II**: $x < 0, y > 0$
- **Quadrant III**: $x < 0, y < 0$
- **Quadrant IV**: $x > 0, y < 0$

## 2. Horizontal and Vertical Displacements

Given two points $P_1(x_1, y_1)$ and $P_2(x_2, y_2)$:
- **Horizontal displacement**: $\Delta x = x_2 - x_1$
- **Vertical displacement**: $\Delta y = y_2 - y_1$

When two points share the same $y$-coordinate ($y_1 = y_2$), the line connecting them is horizontal:
$$d = |x_2 - x_1|$$
When two points share the same $x$-coordinate ($x_1 = x_2$), the line connecting them is vertical:
$$d = |y_2 - y_1|$$

## 3. The Euclidean Distance Formula

When neither displacement is zero, the line segment between $P_1$ and $P_2$ forms the hypotenuse of a right triangle whose legs are the horizontal and vertical displacements $|\Delta x|$ and $|\Delta y|$.

By the Pythagorean theorem:
$$d^2 = (\Delta x)^2 + (\Delta y)^2 = (x_2 - x_1)^2 + (y_2 - y_1)^2$$

Taking the positive square root gives the Euclidean distance:
$$d(P_1, P_2) = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2}$$

### Comparison with Manhattan (Taxicab) Distance

If motion is constrained to grid lines:
$$d_1 = |\Delta x| + |\Delta y|$$

By the triangle inequality:
$$d_{\text{Euclidean}} \le d_{\text{Manhattan}}$$
Strict equality holds if and only if the points lie along the same horizontal or vertical line.
