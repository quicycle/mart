from arpy import ARContext


# 'natural' orderings
ALLOWED_I = "p 23 31 12 0 023 031 012 123 1 2 3 0123 01 02 03".split()
# flipped fields
ALLOWED_II = "p 32 13 21 0 023 031 012 123 1 2 3 0123 10 20 30".split()
# all flipped
ALLOWED_III = "p 32 13 21 0 032 013 021 321 1 2 3 0123 10 20 30".split()
# flipped trivectors
ALLOWED_IV = "p 23 31 12 0 032 013 021 321 1 2 3 0123 01 02 03".split()

# # 'natural' orderings
# ALLOWED_I = "p 23 31 12 0 023 031 012 123 1 2 3 1230 01 02 03".split()
# # flipped fields
# ALLOWED_II = "p 32 13 21 0 023 031 012 123 1 2 3 1230 10 20 30".split()
# # all flipped
# ALLOWED_III = "p 32 13 21 0 032 013 021 321 1 2 3 1230 10 20 30".split()
# # flipped trivectors
# ALLOWED_IV = "p 23 31 12 0 032 013 021 321 1 2 3 1230 01 02 03".split()


MAXWELL_SUPPORTING = {
    "positive_I": ARContext(allowed=ALLOWED_I, metric="+---", div="into"),
    "positive_II": ARContext(allowed=ALLOWED_II, metric="-+++", div="by"),
    "positive_III": ARContext(allowed=ALLOWED_III, metric="+---", div="by"),
    "positive_IV": ARContext(allowed=ALLOWED_IV, metric="-+++", div="into"),
    "negative_I": ARContext(allowed=ALLOWED_I, metric="-+++", div="by"),
    "negative_II": ARContext(allowed=ALLOWED_II, metric="+---", div="into"),
    "negative_III": ARContext(allowed=ALLOWED_III, metric="-+++", div="into"),
    "negative_IV": ARContext(allowed=ALLOWED_IV, metric="+---", div="by"),
}

for name, algebra in MAXWELL_SUPPORTING.items():
    print(name)
    algebra.decompose()
    print()
