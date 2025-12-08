from tool.runners.python import SubmissionPy

import math
from functools import cache, reduce
from itertools import combinations
import networkx as nx
from operator import mul


class ThChSubmission(SubmissionPy):
    def run(self, s: str, n=1000):
        """
        :param s: input in string format
        :return: solution flag
        """
        boxes = []
        for line in s.splitlines():
            x, y, z = line.split(",")
            boxes.append((int(x), int(y), int(z)))

        @cache
        def dist(p1, p2):
            return math.sqrt((p2[0]-p1[0])**2 + (p2[1]-p1[1])**2 + (p2[2]-p1[2])**2)

        distances = sorted((dist(p1, p2), p1, p2) for p1, p2 in combinations(boxes, 2))

        pairs = set()
        circuits = nx.Graph()
        i = 0

        for _ in range(n):
            while True:
                _, p1, p2 = distances[i]
                if (p1, p2) not in pairs:
                    pairs.add((p1, p2))
                    break
                else:
                    i += 1
            circuits.add_edge(p1, p2)

        largest_circuits = sorted(nx.connected_components(circuits), key=len, reverse=True)
        return reduce(mul, [circuits.subgraph(largest_circuits[i]).number_of_nodes() for i in range(3)])


def test_th_ch():
    """
    Run `python -m pytest ./day-08/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
""".strip(), n=10
        )
        == 40
    )
