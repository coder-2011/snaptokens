import unittest

from score import paired_center


class PairedCenterTests(unittest.TestCase):
    def test_even_pairs_are_centered_in_log_space(self):
        self.assertAlmostEqual(paired_center([0.5, 2.0]), 1.0)

    def test_swapping_parent_and_candidate_inverts_result(self):
        ratios = [0.6, 0.8, 1.1, 1.3, 1.5, 2.4]
        self.assertAlmostEqual(
            paired_center(ratios) * paired_center([1 / value for value in ratios]), 1.0
        )


if __name__ == "__main__":
    unittest.main()
