from tool.runners.python import SubmissionPy

import networkx as nx


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        G = nx.DiGraph()
        for line in s.splitlines():
            from_device, to_devices = line.split(": ")
            to_devices = to_devices.split(" ")
            for to_device in to_devices:
                G.add_edge(from_device, to_device)

        return len(list(nx.all_simple_paths(G, source="you", target="out")))


def test_th_ch():
    """
    Run `python -m pytest ./day-11/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
""".strip()
        )
        == 5
    )
