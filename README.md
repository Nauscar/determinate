# Determinate
Attributes to mark a function as determinate or indeterminate.

All determinate functions are run as n many futures, where n is the total number of cores available, and their returned values compared.

When an indeterminate function is called, only the first value returned is used for all futures being executed.

Any errors found using this runtime can indicate an underlying bug effecting the determinism of a function.