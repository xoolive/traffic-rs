from __future__ import annotations

from datetime import datetime, timezone
from typing import Any, Iterator

import pandas as pd

from ._rust_core import interval_add, interval_and
from .time import timelike, to_datetime


class Interval:
    start: datetime
    stop: datetime

    def __init__(self, start: float | datetime, stop: float | datetime) -> None:
        start = (
            datetime.fromtimestamp(start, timezone.utc)
            if isinstance(start, (int, float))
            else start
        )
        stop = (
            datetime.fromtimestamp(stop, timezone.utc)
            if isinstance(stop, (int, float))
            else stop
        )
        self.start = start
        self.stop = stop
        if self.start > self.stop:
            raise RuntimeError("Start value should be anterior to stop value")

    def __repr__(self) -> str:
        return f"[{self.start}, {self.stop}]"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, Interval):
            return False
        return self.start == other.start and self.stop == other.stop

    def __and__(self, other: Interval) -> None | Interval:
        """Returns the intersection between two intervals.

        >>> i1 = Interval(1647861000, 1647861120)
        >>> i2 = Interval(1647861060, 1647861180)
        >>> i1 & i2
        [2022-03-21 11:11:00+00:00, 2022-03-21 11:12:00+00:00]

        """

        if isinstance(other, Interval):
            res = interval_and(
                int(self.start.timestamp()),
                int(self.stop.timestamp()),
                int(other.start.timestamp()),
                int(other.stop.timestamp()),
            )
            # print(self, other, res)
            if res.get("empty", None):
                return None
            return Interval(res["start"], res["stop"])

        return NotImplemented

    def __add__(self, other: Interval) -> IntervalCollection:
        """Concatenates the two elements in an IntervalCollection.

        >>> i1 = Interval(1647861000, 1647861120)
        >>> i2 = Interval(1647861060, 1647861180)
        >>> i1 + i2
        [[2022-03-21 11:10:00+00:00, 2022-03-21 11:13:00+00:00]]

        """
        if isinstance(other, Interval):
            res = interval_add(
                int(self.start.timestamp()),
                int(self.stop.timestamp()),
                int(other.start.timestamp()),
                int(other.stop.timestamp()),
            )
            return IntervalCollection(list(res["start"]), list(res["stop"]))

        return NotImplemented


class IntervalCollection:
    """A class to represent collections of Intervals.

    An :class:`~Interval` consists of a start and stop attributes.
    Collections of intervals are stored as a :class:`~pandas.DataFrame`.

    Intervals can be created using one of the following syntaxes:

    >>> sample_dates = pd.date_range("2023-01-01", "2023-02-01", freq="1D")
    >>> t0, t1, t2, t3, *_ = sample_dates

    - as a list of :class:`~Interval`:

        >>> IntervalCollection([Interval(t0, t1), Interval(t2, t3)])
        [[2023-01-01 00:00:00, 2023-01-02 00:00:00], ...]

    - as an expanded tuple of :class:`~Interval`:

        >>> IntervalCollection(Interval(t0, t1), Interval(t2, t3))
        [[2023-01-01 00:00:00, 2023-01-02 00:00:00], ...]

    - a list of start and stop values:

        >>> IntervalCollection([t0, t2], [t1, t3])
        [[2023-01-01 00:00:00, 2023-01-02 00:00:00], ...]

    - as a :class:`~pandas.DataFrame`:

        >>> df = pd.DataFrame({'start': [t0, t2], 'stop': [t1, t3]})
        >>> IntervalCollection(df)
        [[2023-01-01 00:00:00, 2023-01-02 00:00:00], ...]

    """

    data: pd.DataFrame

    def __init__(
        self,
        data: None
        | pd.DataFrame
        | Interval
        | list[Interval]
        | timelike
        | list[timelike] = None,
        *other: Interval | timelike | list[timelike],
        start: None | timelike | list[timelike] = None,
        stop: None | timelike | list[timelike] = None,
    ) -> None:
        if isinstance(data, Interval):
            data = [data, *other]
        if isinstance(data, list):
            if all(isinstance(elt, Interval) for elt in data):
                # TODO redo
                start = [elt.start for elt in data]
                stop = [elt.stop for elt in data]
                data = None
        if not isinstance(data, pd.DataFrame):
            # We reorder parameters here to accept notations like
            # IntervalCollection(start, stop)
            if start is None or stop is None:
                start, stop, *_ = data, *other, start, stop
                data = None
        if data is None:
            if start is None or stop is None:
                msg = "If no data is specified, provide start and stop"
                raise TypeError(msg)
            if isinstance(start, (str, float, datetime, pd.Timestamp)):
                start = [start]
            if isinstance(stop, (str, float, datetime, pd.Timestamp)):
                stop = [stop]
            assert isinstance(start, list)
            assert isinstance(stop, list)
            if len(start) == 0 or len(stop) == 0:
                msg = "If no data is specified, provide start and stop"
                raise TypeError(msg)

            data = pd.DataFrame(
                {
                    "start": [to_datetime(t) for t in start],
                    "stop": [to_datetime(t) for t in stop],
                }
            )

        # assert isinstance(data, pd.DataFrame)
        # assert data.eval("(start > stop).sum()") == 0

        self.data = data

    def __iter__(self) -> Iterator[Interval]:
        for _, line in self.data.iterrows():
            yield Interval(line.start, line.stop)

    def __repr__(self) -> str:
        return repr(list(i for i in self))
