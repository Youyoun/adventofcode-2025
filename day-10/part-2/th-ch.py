from tool.runners.python import SubmissionPy

from scipy.optimize import linprog


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        total = 0
        for line in s.splitlines():
            data = line.split()
            buttons = [set(map(int, button[1:-1].split(","))) for button in data[1:-1]]
            joltage_levels = list(map(int, data[-1][1:-1].split(",")))

            # Require the total joltage levels to match the goal
            coefficients_equalities = [
                [int(i in button) for button in buttons] for i in range(len(joltage_levels))
            ]
            constants_equalities = joltage_levels

            # Require each button press count to be >= 0
            bounds = (0, None)

            # Minimize 1 * button1 + 1 * button2 + ...
            coefficients_min = [1] * len(buttons)

            res = linprog(coefficients_min,
                A_eq=coefficients_equalities,
                b_eq=constants_equalities,
                bounds=(bounds,) * len(buttons),
                integrality=1, # Force integer solutions
            )
            total += int(res.fun)

        return total


def test_th_ch():
    """
    Run `python -m pytest ./day-10/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
""".strip()
        )
        == 33
    )
