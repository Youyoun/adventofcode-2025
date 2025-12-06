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
            offset =  int(line[1:])
            while offset > 0:
                missing_to_zero = dial if line[0] == "L" else 100 - dial
                move = min(offset, missing_to_zero) if dial != 0 else min(offset, 100)
                dial += (-1 if line[0] == "L" else 1) * move
                dial %= 100
                if dial == 0:
                    password += 1
                offset -= move

        return password


def test_th_ch():
    """
    Run `python -m pytest ./day-01/part-2/th-ch.py` to test the submission.
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
        == 6
    )
    assert (
        ThChSubmission().run(
            """
R1000
""".strip()
        )
        == 10
    )
    assert (
        ThChSubmission().run(
            """
L150
""".strip()
        )
        == 2
    )
    assert (
        ThChSubmission().run(
            """
L50
R2
L3
""".strip()
        )
        == 2
    )
