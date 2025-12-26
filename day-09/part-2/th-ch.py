from tool.runners.python import SubmissionPy

from math import inf
from itertools import combinations
from shapely import Polygon
from shapely.geometry import box


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        tiles = [tuple(map(int, line.split(","))) for line in s.splitlines()]
        area = lambda a, b: (abs(b[0]-a[0])+1) * (abs(b[1]-a[1])+1)
        polygon = Polygon(tiles)
        max_rect = -inf

        for c1, c3 in combinations(tiles, 2):
            c2 = (c3[0], c1[1])
            c4 = (c1[0], c3[1])
            rect = Polygon([c1,c2,c3,c4])
            intersection = polygon.intersection(rect)
            box_around_intersection = box(*intersection.bounds)
            if intersection.area == box_around_intersection.area:
                max_rect = max(max_rect, area(c1, c3))

        return max_rect


def test_th_ch():
    """
    Run `python -m pytest ./day-09/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
""".strip()
        )
        == 24
    )
