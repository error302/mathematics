# Scientific Notation and Powers of Ten

## 1. Motivation: Scale and Readability

When dealing with astronomical distances or subatomic scales, writing numbers in standard positional form becomes cumbersome and error-prone:
- The mass of Earth: $\approx 5,972,000,000,000,000,000,000,000 \text{ kg}$
- The mass of an electron: $\approx 0.0000000000000000000000000000009109 \text{ kg}$

Scientific notation provides an exact, concise representation:
$$\text{Earth mass} \approx 5.972 \times 10^{24} \text{ kg}$$
$$\text{Electron mass} \approx 9.109 \times 10^{-31} \text{ kg}$$

## 2. Standard Form Definition

A number is in **standard scientific notation** if written as:
$$m \times 10^k$$
where:
- $m$ is the **significand** (or mantissa), requiring $1 \le |m| < 10$.
- $k$ is an integer (the exponent/power of ten).

### Converting to Scientific Notation
1. Place the decimal point after the first non-zero digit to form $m$.
2. Count the number of places $k$ the decimal point moved:
   - If the original number is $\ge 10$, $k$ is positive.
   - If the original number is $< 1$, $k$ is negative.

## 3. Multiplication and Division in Scientific Notation

Take advantage of commutativity and exponent laws:
$$(a \times 10^p) \times (b \times 10^q) = (a \times b) \times 10^{p+q}$$

If the resulting product $a \times b \ge 10$, re-normalize:
$$(4 \times 10^5) \times (3 \times 10^6) = 12 \times 10^{11} = 1.2 \times 10^{12}$$
