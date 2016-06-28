# interleave-rs #

Interleave is a macro that allows you to create an iterator that interleaves its input iterators.

The reason for making this library is because Itertools only has binary interleaving.

# Behaviour #

All iterators are exhausted (return None) before the interleaver will return None.
