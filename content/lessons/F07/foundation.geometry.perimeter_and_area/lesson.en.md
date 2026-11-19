# Perimeter, Area, and Spatial Conservation

## 1. Boundary vs Interior

Geometry begins with distinguishing the boundary of a region from the surface it encloses:

- **Perimeter ($P$)**: The total distance traveled along the one-dimensional outline of a closed 2D shape. Measured in linear units ($m$, $cm$, units).
- **Area ($A$)**: The total amount of two-dimensional planar space enclosed within the boundary. Measured in square units ($m^2$, $cm^2$, $\text{units}^2$).

### The Rectangle

For a rectangle with width $w$ and height $h$:
$$P = w + h + w + h = 2(w + h)$$
$$A = w \times h$$

A crucial mathematical insight is that **perimeter and area do not determine each other uniquely**:
- A $6 \times 6$ square has perimeter $P = 24$ and area $A = 36$.
- A $2 \times 10$ rectangle has the same perimeter $P = 24$, but its area is only $A = 20$.
- A $1 \times 11$ rectangle has perimeter $P = 24$, but its area is only $A = 11$.

As a shape becomes more elongated, it requires more perimeter to enclose the same area. The square is the most efficient rectangle for enclosing area with a given perimeter (an instance of the *isoperimetric theorem*).

## 2. Area of Triangles and Dissection

Every triangle with base $b$ and perpendicular altitude $h$ can be enclosed in a bounding rectangle of dimensions $b \times h$.

A perpendicular dropped from the apex splits the triangle into two right triangles. Each right triangle occupies exactly half of its respective half of the bounding rectangle:

$$A_{\text{triangle}} = \frac{1}{2} b h$$

### Shear Invariance (Cavalieri's Principle)

If you slide the apex of a triangle horizontally parallel to its base, the triangle deforms into an oblique triangle. However, neither the base length $b$ nor the perpendicular altitude $h$ changes. Therefore, its **area remains strictly conserved**:

$$\Delta A = 0$$

## 3. Dimensional Scaling Laws

When all linear dimensions of a 2D shape are scaled by a factor of $k$:
- Every side length scales by $k$: $L' = k L$.
- The perimeter scales by $k$: $P' = k P$.
- The area scales by $k^2$: $A' = k^2 A$.

Because area measures covering in two orthogonal independent dimensions ($x$ and $y$), scaling each dimension by $k$ multiplies the tile count by $k \times k = k^2$.
