from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[int(c == "@") for c in line] for line in s.splitlines()]
        total_accessible = 0
        while True:
            accessible = set()
            for y, line in enumerate(grid):
                for x, c in enumerate(line):
                    if c != 1:
                        continue

                    neighbors = sum(grid[y+dy][x+dx] for dx in range(-1, 2) for dy in range(-1, 2) if (dx != 0 or dy != 0) and 0 <= x+dx < len(line) and 0 <= y+dy < len(grid))
                    if neighbors < 4:
                        accessible.add((x, y))

            if not accessible:
                break

            total_accessible += len(accessible)
            for (x, y) in accessible:
                grid[y][x] = 0

        return total_accessible


def test_th_ch():
    """
    Run `python -m pytest ./day-04/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
""".strip()
        )
        == 43
    )
