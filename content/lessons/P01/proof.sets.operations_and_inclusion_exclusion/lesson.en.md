# Set Operations, Power Sets & Principle of Inclusion-Exclusion

## Intuition
A **set** is a well-defined collection of distinct mathematical objects. Two fundamental counting principles govern finite sets:
1. **Binary Choice in Power Sets:** To build a subset $S \subseteq A$ when $|A| = n$, we make a binary decision (include or exclude) for each of the $n$ elements, giving $2^n$ total subsets in the power set $\mathcal{P}(A)$.
2. **Multiplicity Correction (Inclusion-Exclusion):** When sets overlap, summing their sizes overcounts shared elements. Subtracting pairwise intersections and restoring triple intersections guarantees that every element in the union is counted exactly once.

## Formal Definition
1. **Power Set Cardinality:** For $|A| = n$,
   $$|\mathcal{P}(A)| = 2^n, \quad |\{S \subsetneq A\}| = 2^n - 1, \quad |\{S \subsetneq A : S \ne \emptyset\}| = 2^n - 2$$
2. **Principle of Inclusion-Exclusion (2 and 3 Sets):**
   $$|A \cup B| = |A| + |B| - |A \cap B|$$
   $$|A \cup B \cup C| = |A| + |B| + |C| - |A \cap B| - |A \cap C| - |B \cap C| + |A \cap B \cap C|$$

## Worked Example
**Problem:** A set $A$ has $|A| = 6$ elements. How many non-empty proper subsets does $A$ have?

1. **Total subsets in $\mathcal{P}(A)$:** $2^6 = 64$.
2. **Exclude $\emptyset$ and $A$:** Removing the empty set $\emptyset$ and the improper subset $A$ itself leaves $64 - 2 = 62$ non-empty proper subsets.

## Common Misconceptions
- **Forgetting $\emptyset \subseteq A$:** The empty set is a valid proper subset of every non-empty set $A$.
- **Sign of $|A \cap B \cap C|$:** Elements in all three sets are added $\binom{3}{1} = 3$ times and subtracted $\binom{3}{2} = 3$ times, so $|A \cap B \cap C|$ must be **added** back.
