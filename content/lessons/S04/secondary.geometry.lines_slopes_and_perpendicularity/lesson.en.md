# Analytic Lines, Negative-Reciprocal Perpendicularity, and the Section Formula

## 1. Exact Slope and Collinearity
Given two distinct points $A(x_1, y_1)$ and $B(x_2, y_2)$ with $x_1 \neq x_2$, the **slope** $m$ measures the constant ratio of vertical change (rise) to horizontal change (run):
$$m = \frac{y_2 - y_1}{x_2 - x_1} \in \mathbb{Q}$$
Three points $A, B, C$ are **collinear** if and only if the slope of $AB$ equals the slope of $BC$.

## 2. Parallel and Perpendicular Lines
Why do perpendicular lines have **negative-reciprocal** slopes?
If a line moves horizontally by $\Delta x = q$ and vertically by $\Delta y = p$, its slope is $m_1 = \frac{p}{q}$. Rotating that displacement vector $90^\circ$ counterclockwise maps $(q, p) \mapsto (-p, q)$, which has horizontal run $-p$ and vertical rise $q$:
$$m_\perp = \frac{q}{-p} = -\frac{q}{p} = -\frac{1}{m_1} \iff m_1 \cdot m_\perp = -1$$

Given a point $(x_0, y_0)$ on the perpendicular line, its point-slope equation $y - y_0 = m_\perp(x - x_0)$ yields the exact $y$-intercept:
$$b = y_0 - m_\perp x_0$$

## 3. The Internal Section Formula
Suppose point $P$ lies on the directed segment from $A(x_1, y_1)$ to $B(x_2, y_2)$ and partitions it in the ratio $AP : PB = m : n$. Because $P$ lies a fraction $\frac{m}{m+n}$ of the way from $A$ to $B$:
$$x_P = x_1 + \frac{m}{m + n}(x_2 - x_1) = \frac{n x_1 + m x_2}{m + n}, \qquad y_P = \frac{n y_1 + m y_2}{m + n}$$
Notice the **cross-weighting**: endpoint $A(x_1, y_1)$ is weighted by $n$ and endpoint $B(x_2, y_2)$ is weighted by $m$.
