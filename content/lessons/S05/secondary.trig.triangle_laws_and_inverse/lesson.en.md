# Inverse Trigonometric Branches, Law of Sines, and Law of Cosines

## 1. Principal Branches of Inverse Trigonometric Functions
Because trigonometric functions are periodic, they are not one-to-one on $\mathbb{R}$. To define true inverse functions, we restrict each function to a **principal branch** where it is strictly monotonic and bijective onto its range:
- $\arcsin : [-1, 1] \to \left[-\frac{\pi}{2}, \frac{\pi}{2}\right]$ (Quadrants IV and I)
- $\arccos : [-1, 1] \to [0, \pi]$ (Quadrants I and II)
- $\arctan : \mathbb{R} \to \left(-\frac{\pi}{2}, \frac{\pi}{2}\right)$ (Quadrants IV and I)

When evaluating $\arcsin(\sin\theta)$ for $\theta \in \left(\frac{\pi}{2}, \pi\right)$ (Quadrant II), $\theta$ lies outside $\left[-\frac{\pi}{2}, \frac{\pi}{2}\right]$, so we reflect across $\pi/2$ using $\sin(\pi - \theta) = \sin\theta$:
$$\arcsin\!\left(\sin\!\left(\frac{(n-1)\pi}{n}\right)\right) = \pi - \frac{(n-1)\pi}{n} = \frac{1}{n}\pi$$

## 2. The Law of Sines
In any triangle $\triangle ABC$, dropping the altitude $h$ from $C$ to side $c$ gives $h = b\sin A = a\sin B$. Dividing by $\sin A\sin B$ yields the **Law of Sines**:
$$\frac{a}{\sin A} = \frac{b}{\sin B} = \frac{c}{\sin C} \implies b = \frac{a\sin B}{\sin A}$$

## 3. The Law of Cosines
Placing vertex $C$ at the origin with $\vec{a}$ and $\vec{b}$ forming angle $C$, the third side is $\vec{c} = \vec{a} - \vec{b}$. Expanding $\|\vec{a} - \vec{b}\|^2 = \|\vec{a}\|^2 + \|\vec{b}\|^2 - 2\vec{a}\cdot\vec{b}$ gives the **Law of Cosines**:
$$c^2 = a^2 + b^2 - 2ab\cos C \iff \cos C = \frac{a^2 + b^2 - c^2}{2ab}$$
- $\cos C > 0 \iff a^2 + b^2 > c^2$ ($C$ is acute)
- $\cos C = 0 \iff a^2 + b^2 = c^2$ ($C = 90^\circ$, Pythagorean Theorem)
- $\cos C < 0 \iff a^2 + b^2 < c^2$ ($C$ is obtuse)
