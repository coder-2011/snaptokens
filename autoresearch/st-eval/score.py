import math
import statistics


def paired_center(ratios):
    return math.exp(statistics.median(math.log(value) for value in ratios))
