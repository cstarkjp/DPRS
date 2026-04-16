
"""!
@file test_dprs_version.py
@brief Unit test DPRS package version number.
"""

import unittest
import dprs

class TestLangevinVersion(unittest.TestCase):

    def test_langevin_version(self):
        self.assertIn("__version__", dprs.__dict__)
        print(f"DPRS version:  {dprs.__version__}")

if __name__ == '__main__':
    unittest.main()