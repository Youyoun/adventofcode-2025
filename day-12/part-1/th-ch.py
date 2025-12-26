from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        data = s.split("\n\n")
        presents, regions = data[:-1], data[-1].splitlines()
        presents_sizes = [present.count("#") for present in presents]

        valid_regions = 0

        for region in regions:
            dims, counts = region.split(": ")
            w, h = map(int, dims.split("x"))
            counts = list(map(int, counts.split(" ")))

            # If enough 3x3 squares fit in the region, it works
            if sum(counts) * 3 * 3 <= w * h:
                valid_regions += 1
                continue

            # If the region is too small for the present sizes, it's impossible
            if sum(count * presents_sizes[i] for i, count in enumerate(counts)) > w * h:
                continue

            # Input is made so that we don't have to do complex packing
            raise Exception("Not sure if presents fit in the region")

        return valid_regions
