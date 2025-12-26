from tool.runners.python import SubmissionPy

from collections import deque
import networkx as nx


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        total = 0
        for line in s.splitlines():
            data = line.split()
            goal = tuple(light == "#" for light in data[0][1:-1])

            G = nx.Graph()
            start = (False,) * len(goal)
            G.add_node(start)

            buttons = [set(map(int, button[1:-1].split(","))) for button in data[1:-1]]
            q = deque([(start, button) for button in buttons])
            while q:
                state, button = q.popleft()
                new_state = tuple(not light if i in button else light for i, light in enumerate(state))
                if not G.has_node(new_state):
                    q.extend([(new_state, button) for button in buttons])

                G.add_edge(state, new_state, attr={"button": button})

            total += nx.shortest_path_length(G, source=start, target=goal)

        return total


def test_th_ch():
    """
    Run `python -m pytest ./day-10/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
""".strip()
        )
        == 7
    )
