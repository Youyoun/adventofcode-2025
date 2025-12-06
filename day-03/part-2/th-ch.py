from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        joltage = 0
        for bank in s.splitlines():
            batteries = list(map(int, bank))
            turned_on = []

            start = -1
            for remaining in range(11, -1, -1):
                scope = batteries[start+1:len(batteries)-remaining]
                largest = max(scope)
                start = scope.index(largest) + start + 1
                turned_on.append(largest)

            joltage += int("".join(map(str, turned_on)))

        return joltage


def test_th_ch():
    """
    Run `python -m pytest ./day-03/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
987654321111111
811111111111119
234234234234278
818181911112111
""".strip()
        )
        == 3121910778619
    )
