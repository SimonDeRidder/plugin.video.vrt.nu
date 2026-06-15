# -*- coding: utf-8 -*-
# GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
"""Sanity check that native_init wiring + _vrtmax.init agree on the contract."""
import unittest

try:
    import _vrtmax
except ImportError as err:
    raise unittest.SkipTest('_vrtmax not built; run `make dev` first') from err

from native_init import build_settings_dict, init_vrtmax


class NativeInitTest(unittest.TestCase):
    """Tests for initialisation of native_lib."""
    def test_init_and_update_round_trip(self):
        """Test `init` and `update_settings` in round trip."""
        init_vrtmax()

        settings = build_settings_dict()
        self.assertIn('usefavorites', settings)
        self.assertIn('kodi_version_major', settings)
        self.assertIn('has_iptv_manager', settings)

        _vrtmax.update_settings({'usefavorites': False})

        with self.assertRaises(_vrtmax.ParseError):
            _vrtmax.update_settings({'definitely_not_a_real_field': True})

    def test_exception_classes_exist(self):
        """Test existence of exception classes."""
        for name in (
            'AuthError', 'LoginInvalidError', 'LoginEmptyError', 'RefreshTokenError',
            'NetworkError', 'GraphQLError', 'ParseError', 'RateLimitError', 'NotFoundError',
        ):
            self.assertTrue(hasattr(_vrtmax, name), f'missing _vrtmax.{name}')


if __name__ == '__main__':
    unittest.main()
