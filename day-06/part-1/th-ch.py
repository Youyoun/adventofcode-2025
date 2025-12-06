from tool.runners.python import SubmissionPy

from collections import defaultdict
from functools import reduce


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        problems = defaultdict(list)
        for line in s.splitlines():
            for i, x in enumerate(line.split()):
                problems[i].append(x)

        return sum(reduce(lambda x, y: x+y if problem[-1] == "+" else x*y, map(int, problem[:-1])) for problem in problems.values())


def test_th_ch():
    """
    Run `python -m pytest ./day-06/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
""".strip()
        )
        == 4277556
    )
