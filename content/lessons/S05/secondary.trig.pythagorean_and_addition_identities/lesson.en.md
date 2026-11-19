# Pythagorean, Angle-Addition, and Double-Angle Identities

## 1. Pythagorean and Quotient Identities
On a right triangle with legs $a, b$ and hypotenuse $c$ (so $a^2 + b^2 = c^2$), an acute angle $\theta \in (0, \pi/2)$ with $\sin\theta = \frac{a}{c}$ and $\cos\theta = \frac{b}{c}$ has tangent:
$$\tan\theta = \frac{\sin\theta}{\cos\theta} = \frac{a/c}{b/c} = \frac{a}{b}$$
Dividing $\sin^2\theta + \cos^2\theta = 1$ by $\cos^2\theta$ yields $1 + \tan^2\theta = \sec^2\theta$.

## 2. Angle-Addition Formulas
Let $\vec{u} = \langle \cos\alpha, \sin\alpha \rangle$ and $\vec{v} = \langle \cos\beta, -\sin\beta \rangle$ be unit vectors at angles $\alpha$ and $-\beta$. The angle between them is $\alpha - (-\beta) = \alpha + \beta$, so their dot product gives:
$$\cos(\alpha + \beta) = \cos\alpha\cos\beta - \sin\alpha\sin\beta$$
$$\sin(\alpha + \beta) = \sin\alpha\cos\beta + \cos\alpha\sin\beta$$
Notice the opposite sign in the cosine addition law: adding two acute angles increases their total angle and decreases its cosine!

## 3. Double-Angle Identities
Setting $\alpha = \beta = \theta$ in the addition formulas yields the **double-angle identities**:
$$\sin(2\theta) = 2\sin\theta\cos\theta$$
$$\cos(2\theta) = \cos^2\theta - \sin^2\theta = 1 - 2\sin^2\theta = 2\cos^2\theta - 1$$
Never replace $\sin(2\theta)$ with $2\sin\theta$: the factor $\cos\theta$ scales the product so that $|\sin(2\theta)| \le 1$ always holds.
