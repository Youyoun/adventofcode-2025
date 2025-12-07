from tool.runners.python import SubmissionPy

from collections import defaultdict


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        start = lines[0].index("S")
        beams = defaultdict(int)
        beams[start] = 1

        for line in lines[1:]:
            new_beams = defaultdict(int)
            for i in beams:
                if line[i] == "^":
                    new_beams[i-1] += beams[i]
                    new_beams[i+1] += beams[i]
                else:
                    new_beams[i] += beams[i]
            beams = new_beams

        return sum(beams.values())



def test_th_ch():
    """
    Run `python -m pytest ./day-07/part-2/th-ch.py` to test the submission.
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
        == 40
    )
