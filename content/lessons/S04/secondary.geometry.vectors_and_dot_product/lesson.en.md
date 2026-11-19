# Planar Vectors, the Dot Product, Orthogonality, and Projections

## 1. The Algebraic and Geometric Dot Product
Given two planar vectors $\vec{u} = \langle u_1, u_2 \rangle$ and $\vec{v} = \langle v_1, v_2 \rangle$, their difference is $\vec{u} - \vec{v} = \langle u_1 - v_1, u_2 - v_2 \rangle$. Expanding the squared distance between their tips gives:
$$\|\vec{u} - \vec{v}\|^2 = (u_1 - v_1)^2 + (u_2 - v_2)^2 = \|\vec{u}\|^2 + \|\vec{v}\|^2 - 2(u_1 v_1 + u_2 v_2)$$
By the Pythagorean Theorem, $\vec{u}$ and $\vec{v}$ form a right angle if and only if $\|\vec{u} - \vec{v}\|^2 = \|\vec{u}\|^2 + \|\vec{v}\|^2$, which happens precisely when the **dot product**
$$\vec{u} \cdot \vec{v} = u_1 v_1 + u_2 v_2 = \|\vec{u}\|\,\|\vec{v}\|\cos\theta$$
equals zero!

## 2. Orthogonality Criterion
Two vectors $\vec{u} = \langle a, b \rangle$ and $\vec{v} = \langle c, k \rangle$ are **orthogonal** ($\vec{u} \perp \vec{v}$) if and only if:
$$\vec{u} \cdot \vec{v} = ac + bk = 0 \implies k = -\frac{ac}{b} \quad (b \neq 0)$$
Do not confuse orthogonality ($\vec{u} \cdot \vec{v} = 0$) with parallelism ($\frac{a}{c} = \frac{b}{k}$, where one vector is a scalar multiple of the other).

## 3. Scalar Projection Along a Direction Vector
For $\vec{v} \neq \vec{0}$, the unit vector in the direction of $\vec{v}$ is $\hat{v} = \frac{\vec{v}}{\|\vec{v}\|}$. The signed shadow length (**scalar projection**) of $\vec{u}$ along $\vec{v}$ is:
$$\operatorname{comp}_{\vec{v}}(\vec{u}) = \vec{u} \cdot \hat{v} = \frac{\vec{u} \cdot \vec{v}}{\|\vec{v}\|} = \frac{u_1 v_1 + u_2 v_2}{\sqrt{v_1^2 + v_2^2}}$$
When $(v_1, v_2, \|\vec{v}\|)$ is a Pythagorean triple such as $(3, 4, 5)$, $(5, 12, 13)$, or $(8, 15, 17)$, the scalar projection is an exact rational number in $\mathbb{Q}$.
