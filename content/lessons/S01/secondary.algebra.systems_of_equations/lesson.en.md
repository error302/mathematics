# Systems of Linear Equations: Substitution and Elimination

## 1. Simultaneous Constraints and Intersection

A single linear equation in two variables, $a_1 x + b_1 y = c_1$, defines a straight line in the coordinate plane containing infinitely many points $(x, y)$.

A **system of two linear equations** requires $(x, y)$ to satisfy both equations at the same time:

$$\begin{cases} a_1 x + b_1 y = c_1 \\ a_2 x + b_2 y = c_2 \end{cases}$$

When the two lines have different slopes ($a_1 b_2 - a_2 b_1 \neq 0$), they cross at a **unique intersection point** $(x, y)$.

## 2. The Method of Substitution

When one equation already expresses a variable explicitly—such as $y = mx + k$—substitute $(mx + k)$ directly for $y$ in the other equation:

$$ax + b(mx + k) = c \implies (a + bm)x + bk = c$$

This reduces the two-variable system to a single linear equation in $x$.

## 3. The Method of Elimination

When both equations are in standard form and a variable has opposite coefficients—such as $+by$ and $-by$:

$$\begin{cases} a_1 x + b y = c_1 \\ a_2 x - b y = c_2 \end{cases}$$

Adding the two equations term-by-term cancels $y$ immediately:

$$(a_1 + a_2)x = c_1 + c_2 \implies x = \frac{c_1 + c_2}{a_1 + a_2}$$

After finding $x$, substitute it back into either original equation to find $y$ and check both equations.
