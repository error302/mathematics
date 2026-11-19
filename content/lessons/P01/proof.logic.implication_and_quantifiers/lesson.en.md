# Propositional Logic, Implication & Quantifier Negation

## Intuition
Mathematical theorems almost always take the form of a conditional **implication** $P \implies Q$ ("if $P$, then $Q$"). Understanding when $P \implies Q$ is true—and how to negate it—is the key to reading and writing proofs:
- $P \implies Q$ is **false** in only one situation: when the hypothesis $P$ is **true** and the conclusion $Q$ is **false**.
- Whenever $P$ is false, $P \implies Q$ is automatically (**vacuously**) true.

## Formal Definition
1. **Implication & Its Negation:**
   $$(P \implies Q) \equiv (\neg P \lor Q), \qquad \neg(P \implies Q) \equiv (P \land \neg Q)$$
2. **Quantifier Negation Laws:**
   $$\neg(\forall x \in D,\; P(x)) \equiv \exists x \in D,\; \neg P(x)$$
   $$\neg(\exists x \in D,\; P(x)) \equiv \forall x \in D,\; \neg P(x)$$
3. **Inequality Negation:** Over ordered sets, the negation of $A < B$ is $A \ge B$ (not $A > B$).

## Worked Example
**Problem:** Negate the statement $\forall x \in \mathbb{Z},\; 4x + 5 < 29$ and find the smallest integer witness $x$ satisfying the negation.

1. **Flip quantifier and negate predicate:**
   $$\neg(\forall x \in \mathbb{Z},\; 4x + 5 < 29) \equiv \exists x \in \mathbb{Z},\; 4x + 5 \ge 29$$
2. **Solve for the minimal integer witness:**
   $$4x \ge 24 \implies x \ge 6 \implies x_{\min} = 6$$

## Common Misconceptions
- **Ignoring vacuous truth:** On a finite domain $D$, $|\{x \in D : P(x) \implies Q(x)\}| = |D| - |\{x \in D : P(x) \land \neg Q(x)\}|$.
- **Negating $<$ as $>$:** If $4x + 5 = 29$ (at $x = 6$), then $4x + 5 < 29$ is already false, so $x = 6$ is the smallest counterexample.
