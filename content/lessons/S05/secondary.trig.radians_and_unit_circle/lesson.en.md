# Radians, the Unit Circle, Quadrant Signs, and Periodic Waves

## 1. Radian Measure as Arc Length
On the unit circle $x^2 + y^2 = 1$ (radius $R = 1$), the **radian measure** of an angle $\theta$ is the exact signed arc length $s$ swept out along the circumference. Because the circumference of a unit circle is $2\pi$:
$$360^\circ = 2\pi \text{ radians} \iff 180^\circ = \pi \text{ radians} \iff D^\circ = \left(\frac{D}{180}\right)\pi \text{ radians}$$

## 2. Unit-Circle Coordinates and Quadrant Signs
For any real angle $\theta$, the terminal ray from the origin intersects the unit circle $x^2 + y^2 = 1$ at the point:
$$P(\theta) = (\cos\theta, \sin\theta)$$
Because $P(\theta)$ lies on $x^2 + y^2 = 1$, every angle satisfies the **Pythagorean Identity**:
$$\cos^2\theta + \sin^2\theta = 1 \implies \cos\theta = \pm\sqrt{1 - \sin^2\theta}$$
The sign is determined by the quadrant of $\theta$:
- **Quadrant I ($0 < \theta < \pi/2$):** $\cos\theta > 0$, $\sin\theta > 0$
- **Quadrant II ($\pi/2 < \theta < \pi$):** $\cos\theta < 0$, $\sin\theta > 0$
- **Quadrant III ($\pi < \theta < 3\pi/2$):** $\cos\theta < 0$, $\sin\theta < 0$
- **Quadrant IV ($3\pi/2 < \theta < 2\pi$):** $\cos\theta > 0$, $\sin\theta < 0$

## 3. Periodic Sinusoidal Waves
For $f(t) = A\cos(\omega\pi t) + D$:
- **Amplitude:** $|A|$ (half the vertical distance between crest and trough)
- **Midline:** $y = D$
- **Fundamental Period:** A full cycle completes when $\omega\pi T = 2\pi$, giving the exact rational period:
  $$T = \frac{2\pi}{\omega\pi} = \frac{2}{\omega}$$
