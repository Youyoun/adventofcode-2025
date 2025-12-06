from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        sum_invalid_ids = 0
        for interval in s.split(","):
            low, high = map(int, interval.split("-"))
            pattern_size = max(len(str(low)) // 2, 1)
            pattern = int("1" + "0" * (pattern_size-1))
            while True:
                product_id = int(str(pattern) * 2)
                if low <= product_id <= high:
                    sum_invalid_ids += product_id
                elif product_id > high:
                    break
                pattern += 1

        return sum_invalid_ids


def test_th_ch():
    """
    Run `python -m pytest ./day-02/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
""".strip()
        )
        == 1227775554
    )
