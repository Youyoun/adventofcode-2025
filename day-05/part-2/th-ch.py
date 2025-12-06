from tool.runners.python import SubmissionPy

from itertools import chain, combinations


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        ranges, _ = s.split("\n\n")
        ranges = set([tuple(map(int, r.split("-"))) for r in ranges.splitlines()])

        def merge_ranges(r1, r2):
            a = min(r1[0], r2[0])
            c = max(r1[0], r2[0])
            if a == c:
                b = min(r1[1], r2[1])
                d = max(r1[1], r2[1])
            else:
                b = r1[1] if a == r1[0] else r2[1]
                d = r1[1] if c == r1[0] else r2[1]

            if b < c:
                return None
            return (a, max(b, d))

        while True:
            merged = None
            for r1, r2 in combinations(ranges, 2):
                merged = merge_ranges(r1, r2)
                if merged:
                    ranges.remove(r1)
                    ranges.remove(r2)
                    ranges.add(merged)
                    break
            if not merged:
                break

        return sum(b - a + 1 for a, b in ranges)


def test_th_ch():
    """
    Run `python -m pytest ./day-05/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
3-5
10-14
16-20
12-18

1
5
8
11
17
32
""".strip()
        )
        == 14
    )
    assert (
        ThChSubmission().run(
            """
3-10
10-16
12-20
16-20

1
""".strip()
        )
        == 18
    )
    assert (
        ThChSubmission().run(
            """
374683527225097-375251279181585
374683527225097-375251279181585
374683527225097-375000816100458

1
""".strip()
        )
        == 567751956489
    )
