from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        dial = 50
        password = 0
        for line in s.splitlines():
            dial += (-1 if line[0] == "L" else 1) * int(line[1:])
            dial %= 100
            if dial == 0:
                password += 1

        return password


def test_th_ch():
    """
    Run `python -m pytest ./day-01/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
""".strip()
        )
        == 3
    )
