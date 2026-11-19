# Equivalent Fractions: Scaling the Whole

## 1. The Challenge
If one rectangular strip is divided into $2$ equal parts and you shade $1$ part ($\frac{1}{2}$), and an identical strip is divided into $6$ equal parts and you shade $3$ parts ($\frac{3}{6}$), why do both shaded areas cover the exact same physical space?

## 2. Predict Before Calculating
If you multiply both the numerator and denominator of $\frac{2}{3}$ by $4$, what happens to the value of the fraction?
- The value quadruples.
- The value stays exactly the same ($\frac{2}{3} = \frac{8}{12}$).
- The value becomes smaller because the pieces are smaller.

## 3. Mathematical Definition: Equivalence Classes in $\mathbb{Q}$
Formally, the set of rational numbers $\mathbb{Q}$ is constructed from pairs of integers $(a, b)$ with $b \neq 0$ under the equivalence relation $\sim$:
$$(a, b) \sim (c, d) \iff a \cdot d = b \cdot c$$
For any non-zero scaling integer $k \in \mathbb{Z} \setminus \{0\}$:
$$\frac{a \times k}{b \times k} = \frac{a}{b} \times \frac{k}{k} = \frac{a}{b} \times 1 = \frac{a}{b}$$
Because $1$ is the multiplicative identity element, multiplying both terms by $k$ does not alter the rational value; it merely alters the subdivision density of the partition.

## 4. Geometric Partition Interpretation
- The denominator $b$ determines into how many congruent partitions the unit interval $[0, 1]$ is divided.
- Multiplying $b$ by $k$ splits each partition into $k$ sub-partitions.
- Multiplying $a$ by $k$ selects $k$ times as many of these smaller sub-partitions.
- The total measure occupied remains invariant.

## 5. Worked Example: Finding an Equivalent Fraction
Find an equivalent fraction for $\frac{3}{5}$ with denominator $20$:
1. Determine the integer factor by which the denominator was scaled: $20 \div 5 = 4$.
2. Scale the numerator by the identical factor: $3 \times 4 = 12$.
3. Conclude: $\frac{3}{5} = \frac{12}{20}$.

## 6. Non-Example Trap: The Additive Fallacy
A frequent error is adding an integer to both numerator and denominator:
$$\frac{a + k}{b + k} \neq \frac{a}{b} \quad (\text{except when } a = b \text{ or } k = 0)$$
For instance: $\frac{1 + 1}{2 + 1} = \frac{2}{3} \approx 0.667 \neq \frac{1}{2} = 0.5$.
Equivalence is strictly a **multiplicative** scaling property ($\times \frac{k}{k}$), never an additive one.

## 7. Reflection
Equivalent fractions provide the bridge needed for all fraction arithmetic: we can only add, subtract, or compare fractional quantities directly when they share common equal-sized parts.

## Additional learning activities (draft)
**Review status:** Independent human review is pending. These activities do not award certified mastery.

### Prerequisite check
Identify unit fractions and understand that the denominator names the number of equal parts in the whole while the numerator counts chosen parts.

### Representation
Take a paper strip folded into $3$ equal thirds, and shade $2$ of them ($\frac{2}{3}$). Now fold the entire strip in half along its length. Each third is split into $2$ smaller pieces: there are now $6$ equal pieces in total, and $4$ are shaded ($\frac{4}{6}$). The shaded length did not change: $\frac{2}{3} = \frac{4}{6}$.

### Complete a partly worked problem
Find the equivalent fraction: $\frac{3}{4} = \frac{3 \times 3}{4 \times \underline{\hspace{1cm}}} = \frac{\underline{\hspace{1cm}}}{12}$.
Self-check: $\frac{3}{4} = \frac{3 \times 3}{4 \times 3} = \frac{9}{12}$.

### Independent practice
1. Find an equivalent fraction for $\frac{2}{5}$ with denominator $15$.
2. Explain why $\frac{4}{10}$ and $\frac{2}{5}$ represent the same rational number.

### Explanation
Explain why multiplying a fraction's top and bottom by $5$ is mathematically equivalent to multiplying the fraction by the number $1$.

### Transfer
Two recipes need sugar. Recipe A asks for $\frac{3}{4}$ cup. Recipe B asks for $\frac{9}{12}$ cup. A baker claims they need different amounts. Are they correct?
Solution: The baker is incorrect. $\frac{9}{12}$ simplified by dividing top and bottom by $3$ gives $\frac{3}{4}$. Both recipes require the exact same amount of sugar.

### Retrieval
Tomorrow find three equivalent fractions for $\frac{5}{6}$. Next week explain why adding $3$ to top and bottom changes the value of $\frac{1}{2}$, whereas multiplying by $3$ preserves it.
