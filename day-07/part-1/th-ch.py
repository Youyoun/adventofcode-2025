from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        beams = set([lines[0].index("S")])
        split = 0

        for line in lines[1:]:
            new_beams = set()
            for i in beams:
                if line[i] == "^":
                    new_beams.add(i-1)
                    new_beams.add(i+1)
                    split += 1
                else:
                    new_beams.add(i)
            beams = new_beams

        return split


def test_th_ch():
    """
    Run `python -m pytest ./day-07/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
""".strip()
        )
        == 21
    )
