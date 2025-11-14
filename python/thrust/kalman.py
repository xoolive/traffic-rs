from typing import TypedDict

import numpy as np
import numpy.typing as npt
import pandas as pd
import polars as pl

from .core import kalman6d_rs


class KalmanResult(TypedDict):
    x_cor: list[npt.NDArray[np.float64]]


def kalman6d(df: pd.DataFrame) -> KalmanResult:
    return kalman6d_rs(pl.from_pandas(df))
