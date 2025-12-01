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
            counts += step // 100
            step = step % 100
            new_dial = dial + step * (-1 if direction == "L" else 1)
            counts += dial != 0 and (new_dial <= 0 or new_dial >= 100)
            dial = new_dial % 100
        return counts

def test_youyoun():
    """
    Run `python -m pytest ./day-01/part-2/youyoun.py` to test the submission.
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
        == 6
    )
