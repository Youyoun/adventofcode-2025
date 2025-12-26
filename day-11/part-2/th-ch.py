from tool.runners.python import SubmissionPy

from functools import cache
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

        @cache
        def dfs(node, seen_fft=False, seen_dac=False):
            if node == "out":
                return int(seen_fft and seen_dac)

            total_paths = 0
            for neighbor in G.neighbors(node):
                total_paths += dfs(neighbor, seen_fft or neighbor == "fft", seen_dac or neighbor == "dac")
            return total_paths

        return dfs("svr")


def test_th_ch():
    """
    Run `python -m pytest ./day-11/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
""".strip()
        )
        == 2
    )
