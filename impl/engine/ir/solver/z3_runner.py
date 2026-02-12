from z3 import Solver


def prove(constraints):
s = Solver()
for c in constraints:
s.add(c)
return s.check().r == 1