from typing import TypedDict

import numpy as np
import numpy.typing as npt
import pandas as pd
import polars as pl
from traffic.algorithms.filters.kalman import ProcessXYZFilterBase

from .core import kalman6d_rs


class KalmanResult(TypedDict):
    x_cor: list[npt.NDArray[np.float64]]


def kalman6d(df: pd.DataFrame) -> KalmanResult:
    return kalman6d_rs(pl.from_pandas(df))


class KalmanFilter6DRust(ProcessXYZFilterBase):
    def apply(self, data: pd.DataFrame) -> pd.DataFrame:
        df = self.preprocess(data)
        res = kalman6d_rs(pl.from_pandas(df))

        filtered = pd.DataFrame(
            res["x_cor"],
            columns=["x", "y", "z", "dx", "dy", "dz"],
        )

        return data.assign(**self.postprocess(filtered))
