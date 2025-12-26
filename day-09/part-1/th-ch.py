from tool.runners.python import SubmissionPy

from itertools import combinations


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        tiles = [tuple(map(int, line.split(","))) for line in s.splitlines()]

        area = lambda a,b: (abs(b[0]-a[0])+1) * (abs(b[1]-a[1])+1)
        return max(area(a, b) for a, b in combinations(tiles, 2))


def test_th_ch():
    """
    Run `python -m pytest ./day-09/part-1/th-ch.py` to test the submission.
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
        == 50
    )
