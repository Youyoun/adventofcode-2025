from tool.runners.python import SubmissionPy


class YouyounSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        dial = 50
        counts = 0
        for line in s.splitlines():
            direction, step = line[0], int(line[1:])
            sign = -1 if direction == "L" else 1
            dial = (dial + step * sign) % 100
            counts += (dial == 0)
        return counts

def test_youyoun():
    """
    Run `python -m pytest ./day-01/part-1/youyoun.py` to test the submission.
    """
    assert (
        YouyounSubmission().run(
            """L68
L30
R48
L5
R60
L55
L1
L99
R14
L82""".strip()
        )
        == 3
    )
