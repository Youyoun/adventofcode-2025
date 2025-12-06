from tool.runners.python import SubmissionPy

from collections import defaultdict
from functools import reduce


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        operators = []
        for i, c in enumerate(lines[-1]):
            if c != " ":
                operators.append((c, i))

        grand_total = 0
        for i, (operator, start) in enumerate(operators):
            end = operators[i+1][1] if i +1 < len(operators) else None
            numbers = defaultdict(list)
            for line in lines[:-1]:
                for j in range(start, end or len(line)):
                    if line[j] != " ":
                        numbers[j].append(line[j])

            numbers = [int("".join(number)) for number in numbers.values()]
            grand_total += reduce(lambda x, y: x+y if operator == "+" else x*y, numbers)

        return grand_total



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
        == 3263827
    )
