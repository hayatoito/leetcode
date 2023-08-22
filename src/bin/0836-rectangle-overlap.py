# @leetup=info id=836 lang=python3 slug=rectangle-overlap



# @leetup=code
class Solution:
    def isRectangleOverlap(self, rec1: list[int], rec2: list[int]) -> bool:
        def is_segment_overlap(i1, i2):
            return i1[0] < i2[1] and i2[0] < i1[1]

        return is_segment_overlap(
            (rec1[0], rec1[2]), (rec2[0], rec2[2])
        ) and is_segment_overlap((rec1[1], rec1[3]), (rec2[1], rec2[3]))


# @leetup=code
