# Angles, Parallel Lines, and Triangle Invariants

## 1. Angle as Circular Rotation

An angle measures the amount of planar rotation between two rays sharing a common origin:
- A full circular rotation is $360^\circ$ ($2\pi\text{ rad}$).
- A straight angle (a straight line) is $180^\circ$ ($\pi\text{ rad}$).
- A right angle (perpendicular lines) is $90^\circ$ ($\pi/2\text{ rad}$).

Two angles are:
- **Complementary** if they sum to $90^\circ$: $\alpha + \beta = 90^\circ$.
- **Supplementary** if they form a linear pair and sum to $180^\circ$: $\alpha + \beta = 180^\circ$.

## 2. Parallel Transversals and the Alternate Interior Angle Theorem

When a transversal line intersects two parallel lines $L_1 \parallel L_2$:
1. **Corresponding angles** are congruent ($\angle 1 \cong \angle 5$).
2. **Alternate interior angles** are congruent ($\angle 3 \cong \angle 5$, $\angle 4 \cong \angle 6$).
3. **Consecutive interior angles** are supplementary ($\angle 4 + \angle 5 = 180^\circ$).

This equivalence is deeply tied to Euclid's fifth postulate (the parallel postulate).

## 3. The Triangle Angle Sum Theorem

For any triangle with interior angles $\alpha, \beta, \gamma$:

$$\alpha + \beta + \gamma = 180^\circ$$

### Deductive Proof

1. Let $\triangle ABC$ have interior angles $\alpha$ at $A$, $\beta$ at $B$, and $\gamma$ at $C$.
2. Through vertex $A$, construct an auxiliary line $L$ strictly parallel to the opposite base $BC$.
3. Since $L \parallel BC$, the side $AB$ acts as a transversal. By the alternate interior angle theorem, the angle formed between line $L$ and side $AB$ equals $\beta$.
4. Similarly, side $AC$ acts as a transversal, so the angle between line $L$ and side $AC$ equals $\gamma$.
5. The three angles $\beta$, $\alpha$, and $\gamma$ now lie adjacent to each other along the straight line $L$.
6. Because angles along a straight line sum to $180^\circ$:
   $$\beta + \alpha + \gamma = 180^\circ$$

### The Exterior Angle Theorem

Extending side $BC$ past $C$ forms an exterior angle $\theta_{\text{ext}}$.
Because $\theta_{\text{ext}} + \gamma = 180^\circ$ and $\alpha + \beta + \gamma = 180^\circ$, it follows that:
$$\theta_{\text{ext}} = \alpha + \beta$$
An exterior angle of a triangle is always equal to the sum of the two remote interior angles.
