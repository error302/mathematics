# Percentages: Parts Per Hundred and Benchmark Calculations

**Course:** F05 — Decimals, Percentages, and Proportional Reasoning  
**Status:** Draft (`review_status: independent_review_pending`)  
**Domain:** Rational Numbers $\mathbb{Q}$

---

## 1. What is a Percentage?

The word *percent* originates from the Latin *per centum*, meaning "by the hundred."
The percent symbol $\%$ is an arithmetic operator that multiplies by $\frac{1}{100}$:
$$p\% = \frac{p}{100} = p \times 0.01$$

Thus, percentages are not a mysterious separate mathematical domain; they are simply fractions with a fixed denominator of $100$.

| Percentage | Fraction | Canonical Lowest Terms | Decimal |
| :--- | :--- | :--- | :--- |
| $100\%$ | $\frac{100}{100}$ | $1$ | $1.0$ |
| $75\%$ | $\frac{75}{100}$ | $\frac{3}{4}$ | $0.75$ |
| $50\%$ | $\frac{50}{100}$ | $\frac{1}{2}$ | $0.5$ |
| $25\%$ | $\frac{25}{100}$ | $\frac{1}{4}$ | $0.25$ |
| $10\%$ | $\frac{10}{100}$ | $\frac{1}{10}$ | $0.1$ |
| $5\%$ | $\frac{5}{100}$ | $\frac{1}{20}$ | $0.05$ |
| $1\%$ | $\frac{1}{100}$ | $\frac{1}{100}$ | $0.01$ |

---

## 2. Benchmark Reasoning: Mental Decomposition

Rather than carrying out cumbersome multiplication by $\frac{p}{100}$ every time, flexible mathematical reasoning constructs arbitrary percentages by combining key benchmarks:

- **$10\%$:** Shift the decimal point one place to the left (divide by $10$).
- **$5\%$:** Half of $10\%$.
- **$50\%$:** Divide by $2$.
- **$25\%$:** Divide by $4$ (or half of $50\%$).
- **$75\%$:** $50\% + 25\%$ or $3 \times 25\%$.

### Example: Calculate $35\%$ of $80$
$$\begin{aligned}
10\% \text{ of } 80 &= 8 \\
30\% \text{ of } 80 &= 3 \times 8 = 24 \\
5\% \text{ of } 80 &= \frac{8}{2} = 4 \\
35\% \text{ of } 80 &= 24 + 4 = 28
\end{aligned}$$

---

## 3. Reversing Percentages

If $25\%$ of an unknown quantity $X$ is $18$, what is $X$?
Since $25\% = \frac{1}{4}$, we have:
$$\frac{1}{4} X = 18 \implies X = 18 \times 4 = 72$$

In general, to find the whole from a part $Y = p\% \times X$:
$$X = \frac{Y}{p / 100} = \frac{100 \cdot Y}{p}$$

---

## 4. Misconception Alert: Percentage Changes Are Asymmetric

If an asset increases in value by $10\%$ and subsequently decreases by $10\%$, does it return to its original value?
Let the initial value be $100$:
1. Increase by $10\%$: $100 + 10 = 110$.
2. Decrease by $10\%$ of **110**: $110 - 11 = 99 \neq 100$.

The second percentage acts upon a different, larger base! Percentages must always be tied to their explicit reference base.
