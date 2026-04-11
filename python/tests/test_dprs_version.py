
"""!
@file test_langevin_version.py
@brief Unit test Langevin package version number.
"""

import unittest
import dprs

class TestLangevinVersion(unittest.TestCase):

    def test_langevin_version(self):
        self.assertIn("__version__", dprs.__dict__)
        print(f"langevin version:  {dprs.__version__}")

if __name__ == '__main__':
    unittest.main()