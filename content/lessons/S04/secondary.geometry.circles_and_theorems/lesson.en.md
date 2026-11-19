# Circle Theorems, Completing the Square, and Power of a Point

## 1. The Inscribed Angle Theorem
Let $A, B, C$ be points on a circle with center $O$. Draw the diameter from $C$ through $O$. Because $OA = OB = OC = R$, triangles $\triangle OAC$ and $\triangle OBC$ are isosceles, so their exterior angles at $O$ equal twice their base angles at $C$. Adding (or subtracting) the two parts proves the **Inscribed Angle Theorem**:
$$\angle ACB = \frac{1}{2} \angle AOB$$
- **Corollary 1 (Angles subtending the same arc):** Any two inscribed angles $\angle ACB$ and $\angle ADB$ subtending the same arc $AB$ are equal.
- **Corollary 2 (Thales's Theorem):** If $AB$ is a diameter ($\angle AOB = 180^\circ$), then $\angle ACB = 90^\circ$.

## 2. Completing the Square for the Circle Equation
By the Pythagorean distance formula, the locus of points $(x, y)$ at distance $R$ from $(h, k)$ is:
$$(x - h)^2 + (y - k)^2 = R^2$$
Expanding yields the general form $x^2 + y^2 - 2hx - 2ky + F = 0$, where $F = h^2 + k^2 - R^2$. To recover the center $(h, k)$ and squared radius $R^2$, complete the square on both variables and add $h^2 + k^2$ to both sides:
$$(x - h)^2 + (y - k)^2 = h^2 + k^2 - F \implies R^2 = h^2 + k^2 - F$$

## 3. Power of a Point (Intersecting Chords Theorem)
Suppose two chords $AB$ and $CD$ intersect at an interior point $P$. Because $\angle ACD$ and $\angle ABD$ subtend the same arc $AD$, they are equal; and vertical angles $\angle APC$ and $\angle DPB$ are equal. Therefore $\triangle PAC \sim \triangle PDB$ by AA similarity, giving:
$$\frac{PA}{PD} = \frac{PC}{PB} \iff PA \cdot PB = PC \cdot PD$$
