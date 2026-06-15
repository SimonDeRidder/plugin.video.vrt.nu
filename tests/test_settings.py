# -*- coding: utf-8 -*-
# GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
"""Integration tests for changes to settings"""

import os
import unittest

from addon import plugin
from kodiutils import open_settings
from streamservice import StreamService
from tokenresolver import TokenResolver

xbmc = __import__('xbmc')
xbmcaddon = __import__('xbmcaddon')
xbmcgui = __import__('xbmcgui')
xbmcplugin = __import__('xbmcplugin')
xbmcvfs = __import__('xbmcvfs')

addon = xbmcaddon.Addon()

PLUGIN_BASE_URL = "plugin://plugin.video.vrt.nu"


class TestSettings(unittest.TestCase):
    """TestCase class"""

    _tokenresolver = TokenResolver()
    _streamservice = StreamService(_tokenresolver)

    def tearDown(self):
        """Clean up function for TestCase class"""
        addon.settings['showfanart'] = True
        addon.settings['showoneoff'] = True
        addon.settings['showyoutube'] = True
        addon.settings['usedrm'] = True
        addon.settings['usefavorites'] = True
        addon.settings['usehttpcaching'] = True
        addon.settings['useinputstreamadaptive'] = True
        addon.settings['usemenucaching'] = True
        addon.settings['useresumepoints'] = True

    @staticmethod
    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_menucaching_disabled():
        """Test without menu caching"""
        addon.settings['usehttpcaching'] = True
        addon.settings['usemenucaching'] = False
        plugin.run([f'{PLUGIN_BASE_URL}/recent', '0', ''])
        plugin.run([f'{PLUGIN_BASE_URL}/recent', '0', ''])

    @staticmethod
    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_httpcaching_disabled():
        """Test without http caching"""
        addon.settings['usehttpcaching'] = False
        addon.settings['usemenucaching'] = True
        plugin.run([f'{PLUGIN_BASE_URL}/offline', '0', ''])
        plugin.run([f'{PLUGIN_BASE_URL}/offline', '0', ''])
        plugin.run([f'{PLUGIN_BASE_URL}/tvguide/date/today/canvas', '0', ''])

    @staticmethod
    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_favorites_disabled():
        """Test without favorites"""
        plugin.run([f'{PLUGIN_BASE_URL}/', '0', ''])
        addon.settings['usefavorites'] = False
        addon.settings['useresumepoints'] = True
        plugin.run([f'{PLUGIN_BASE_URL}/', '0', ''])
        plugin.run([f'{PLUGIN_BASE_URL}/favorites', '0', ''])
        plugin.run([f'{PLUGIN_BASE_URL}/favorites/recent', '0', ''])
        plugin.run([f'{PLUGIN_BASE_URL}/favorites/offline', '0', ''])

    @staticmethod
    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_resumepoints_disabled():
        """Test without resumepoints"""
        plugin.run([f'{PLUGIN_BASE_URL}/favorites', '0', ''])
        addon.settings['usefavorites'] = True
        addon.settings['useresumepoints'] = False
        plugin.run([f'{PLUGIN_BASE_URL}/favorites', '0', ''])
        plugin.run([f'{PLUGIN_BASE_URL}/resumepoints/continue', '0', ''])

    @staticmethod
    def test_opensettings():
        """Test open_settings"""
        open_settings()

    @staticmethod
    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_youtube_disabled():
        """Test with showyoutube disabled"""
        addon.settings['showyoutube'] = False
        plugin.run([f'{PLUGIN_BASE_URL}/channels/radio1', '0', ''])

    @staticmethod
    def test_showfanart_disabled():
        """Test with showfanart disabled"""
        addon.settings['showfanart'] = False
        plugin.run([f'{PLUGIN_BASE_URL}/categories', '0', ''])

    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_ondemand_stream(self):
        """Test ondemand stream"""
        addon.settings['usedrm'] = False
        addon.settings['useinputstreamadaptive'] = False
        video = {'video_url': 'https://www.vrt.be/vrtmax/a-z/winteruur/1/winteruur-s1a1/'}
        stream = self._streamservice.get_stream(video)
        # NOTE: Testing live streams only works within Europe
        if os.environ.get('GITHUB_ACTIONS') != 'true':
            self.assertTrue(stream is not None)

    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_ondemand_stream_ia(self):
        """Test with usedrm disabled"""
        addon.settings['usedrm'] = False
        addon.settings['useinputstreamadaptive'] = True
        video = {'video_url': 'https://www.vrt.be/vrtmax/a-z/winteruur/1/winteruur-s1a1/'}
        stream = self._streamservice.get_stream(video)
        # NOTE: Testing live streams only works within Europe
        if os.environ.get('GITHUB_ACTIONS') != 'true':
            self.assertTrue(stream is not None)

    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_ondemand_stream_drm(self):
        """Test with useinputstreamadaptive disabled"""
        addon.settings['usedrm'] = True
        addon.settings['useinputstreamadaptive'] = False
        video = {'video_url': 'https://www.vrt.be/vrtmax/a-z/winteruur/1/winteruur-s1a1/'}
        stream = self._streamservice.get_stream(video)
        # NOTE: Testing live streams only works within Europe
        if os.environ.get('GITHUB_ACTIONS') != 'true':
            self.assertTrue(stream is not None)

    @unittest.skipUnless(addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_ondemand_stream_drm_ia(self):
        """Test with usedrm and useinputstreamadaptive disabled"""
        addon.settings['usedrm'] = True
        addon.settings['useinputstreamadaptive'] = True
        video = {'video_url': 'https://www.vrt.be/vrtmax/a-z/winteruur/1/winteruur-s1a1/'}
        stream = self._streamservice.get_stream(video)
        # NOTE: Testing live streams only works within Europe
        if os.environ.get('GITHUB_ACTIONS') != 'true':
            self.assertTrue(stream is not None)


if __name__ == '__main__':
    unittest.main()
