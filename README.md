# crys10: long cryptarithm generator
Bart Massey

*This program is a work in progress. It does little useful
yet.*

This program attempts to find legal cryptarithms of a given
length *n*. Assumptions are that:

* Both operands of the addition are of length *n*.

* The result of the addition is of length *n* or *n+1*.

* None of the three words involved are the same.

* Collectively the three words can comprise no more than 10
  letters.

There are a lot of candidates: more than 76 million when
*n=14* according to my dictionary.

**To Do:**

Actually try solving each candidate cryptarithm, checking
that there is a unique solution.
