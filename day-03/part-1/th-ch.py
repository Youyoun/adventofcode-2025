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
            largest = max(batteries[:-1])
            second_largest = max(batteries[batteries.index(largest)+1:])
            joltage += int(f"{largest}{second_largest}")

        return joltage


def test_th_ch():
    """
    Run `python -m pytest ./day-03/part-1/th-ch.py` to test the submission.
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
        == 357
    )
