from typing import TypedDict

import numpy as np  # type: ignore[import]
import numpy.typing as npt  # type: ignore[import]
import polars as pl  # type: ignore[import]

class KalmanResult(TypedDict):
    x_cor: list[npt.NDArray[np.float64]]

def kalman6d_rs(data: pl.DataFrame) -> KalmanResult: ...
