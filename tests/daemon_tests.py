from unittest import TestCase
from microphoned.daemon import daemon_main


class TestConsole(TestCase):
    def test_basic(self):
        daemon_main()