# Center, Spread, and the Balance Point of Data

## 1. The Arithmetic Mean as a Physical Fulcrum

Given $n$ numerical observations $x_1, x_2, \dots, x_n$, the **arithmetic mean** $\bar{x}$ is defined by:

$$\bar{x} = \frac{x_1 + x_2 + \dots + x_n}{n} = \frac{1}{n}\sum_{i=1}^{n} x_i$$

Multiplying both sides by $n$ reveals the **total-sum invariance** of the mean:

$$\sum_{i=1}^{n} x_i = n\bar{x}$$

If you redistribute the total sum equally among all $n$ observations, each observation receives $\bar{x}$. Furthermore, the signed deviations from the mean always cancel out completely:

$$\sum_{i=1}^{n} (x_i - \bar{x}) = \sum_{i=1}^{n} x_i - n\bar{x} = 0$$

Geometrically, if you place equal point masses at positions $x_1, \dots, x_n$ along a rigid number line, $\bar{x}$ is the exact **center of mass** where the line balances in equilibrium.

## 2. The Median and Order Statistics

To find the **median**, we first sort the data in non-decreasing order:

$$x_{(1)} \le x_{(2)} \le \dots \le x_{(n)}$$

- If $n$ is **odd**, the median is the unique central observation $x_{((n+1)/2)}$.
- If $n$ is **even**, the median is the arithmetic mean of the two central observations: $\frac{1}{2}\left(x_{(n/2)} + x_{(n/2 + 1)}\right)$.

> **Critical Rule:** Never pick the middle element of an unsorted list. The median is defined only after sorting.

## 3. Robustness vs Sensitivity

Suppose a dataset is $\{10, 12, 14, 16, 18\}$:
- $\bar{x} = \frac{70}{5} = 14$, and the median is $14$.
- If $18$ is replaced by an extreme outlier $68$, the sorted list is $\{10, 12, 14, 16, 68\}$.
- The **median remains $14$** because rank order in the center did not change.
- The **mean shifts to $\frac{120}{5} = 24$** because the fulcrum must move right to balance the large torque of $68$.
