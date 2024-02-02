from typing import TypedDict

import numpy as np
import numpy.typing as npt

class IntervalCollectionDict(TypedDict):
    start: npt.NDArray[np.int64]
    stop: npt.NDArray[np.int64]

class IntervalDict(TypedDict):
    start: int
    stop: int

def interval_and(
    start1: int, stop1: int, start2: int, stop2: int
) -> IntervalDict: ...
def interval_add(
    start1: int, stop1: int, start2: int, stop2: int
) -> IntervalCollectionDict: ...
