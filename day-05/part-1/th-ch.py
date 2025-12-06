from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        ranges, ingredients = s.split("\n\n")
        ranges = [tuple(map(int, r.split("-"))) for r in ranges.splitlines()]

        return sum(any(a <= int(ingredient) <= b for a, b in ranges) for ingredient in ingredients.splitlines())


def test_th_ch():
    """
    Run `python -m pytest ./day-05/part-1/th-ch.py` to test the submission.
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
        == 3
    )
