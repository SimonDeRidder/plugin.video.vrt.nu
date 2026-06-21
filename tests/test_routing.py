# -*- coding: utf-8 -*-
# GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
"""Integration tests for Routing functionality"""

import unittest
from base64 import b64encode
from datetime import datetime, timedelta

import addon
import dateutil.tz

xbmc = __import__('xbmc')
xbmcaddon = __import__('xbmcaddon')
xbmcgui = __import__('xbmcgui')
xbmcplugin = __import__('xbmcplugin')
xbmcvfs = __import__('xbmcvfs')

xbmc_addon = xbmcaddon.Addon()
plugin = addon.plugin
now = datetime.now(dateutil.tz.tzlocal())
lastweek = now + timedelta(days=-7)

PLUGIN_BASE_URL = "plugin://plugin.video.vrt.nu"


class TestRouting(unittest.TestCase):
    """TestCase class"""

    def test_main_menu(self):
        """Main menu: /"""
        addon.run([f'{PLUGIN_BASE_URL}/', '0', ''])
        self.assertEqual(plugin.url_for(addon.main_menu), f'{PLUGIN_BASE_URL}/')

    def test_noop(self):
        """No operation: /noop"""
        addon.run([f'{PLUGIN_BASE_URL}/noop', '0', ''])
        self.assertEqual(plugin.url_for(addon.noop), f'{PLUGIN_BASE_URL}/noop')

    @staticmethod
    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_favorites():
        """Favorites menu: /favorites"""
        addon.run([f'{PLUGIN_BASE_URL}/favorites', '0', ''])
        addon.run([f'{PLUGIN_BASE_URL}/favorites/programs', '0', ''])
        addon.run([f'{PLUGIN_BASE_URL}/favorites/recent', '0', ''])
        addon.run([f'{PLUGIN_BASE_URL}/favorites/offline', '0', ''])
        addon.run([f'{PLUGIN_BASE_URL}/resumepoints/continue', '0', ''])
        # addon.run([f'{PLUGIN_BASE_URL}/favorites/docu', '0', ''])
        # addon.run([f'{PLUGIN_BASE_URL}/favorites/music', '0', ''])

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_episodes_menu(self):
        """Episodes menu: /programs/<program>"""
        addon.run([f'{PLUGIN_BASE_URL}/programs/thuis', '0', ''])
        self.assertEqual(plugin.url_for(addon.programs, program_name='thuis'), f'{PLUGIN_BASE_URL}/programs/thuis')

        list_id = 'o%35|o%2|o%2|p%/a-z/pano/|container%|banner%|2019|b%1|n%1%'
        list_id = f'${b64encode(list_id.encode("utf-8")).decode("utf-8")}'
        addon.run([f'{PLUGIN_BASE_URL}/programs/pano/{list_id}', '0', ''])
        self.assertEqual(plugin.url_for(addon.programs, program_name='pano', list_id=list_id),
                         f'{PLUGIN_BASE_URL}/programs/pano/{list_id}')

        list_id = 'o%35|o%2|o%2|p%/a-z/de-smurfen0/|container%|episodes-list%|2021|b%0|n%1%'
        list_id = f'${b64encode(list_id.encode("utf-8")).decode("utf-8")}'
        addon.run([f'{PLUGIN_BASE_URL}/programs/de-smurfen0/{list_id}/1655824964821', '0', ''])
        self.assertEqual(plugin.url_for(addon.programs, program_name='de-smurfen0', list_id=list_id, end_cursor='1655824964821'),
                         f'{PLUGIN_BASE_URL}/programs/de-smurfen0/{list_id}/1655824964821')

    def test_categories_menu(self):
        """Categories menu: /categories"""
        addon.run([f'{PLUGIN_BASE_URL}/categories', '0', ''])
        self.assertEqual(plugin.url_for(addon.categories), f'{PLUGIN_BASE_URL}/categories')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_categories_tvshow_menu(self):
        """Categories programs menu: /categories/<category>"""
        addon.run([f'{PLUGIN_BASE_URL}/categories/docu', '0', ''])
        self.assertEqual(plugin.url_for(addon.categories, category='docu'), f'{PLUGIN_BASE_URL}/categories/docu')
        addon.run([f'{PLUGIN_BASE_URL}/categories/voor-kinderen', '0', ''])
        self.assertEqual(plugin.url_for(addon.categories, category='voor-kinderen'), f'{PLUGIN_BASE_URL}/categories/voor-kinderen')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_featured_menu(self):
        """Featured menu: /featured"""
        addon.run([f'{PLUGIN_BASE_URL}/featured', '0', ''])
        self.assertEqual(plugin.url_for(addon.featured), f'{PLUGIN_BASE_URL}/featured')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_featured_tvshow_menu(self):
        """Featured programs menu: /featured/<cfeatured>"""
        tvshow_list_id = '$byUzNXxwJS98bGlzdHxiJTB8biUxJQ=='
        addon.run([f'{PLUGIN_BASE_URL}/featured/program_{tvshow_list_id}', '0', ''])
        self.assertEqual(plugin.url_for(addon.featured,
                                        feature=f'program_{tvshow_list_id}'),
                         f'{PLUGIN_BASE_URL}/featured/program_{tvshow_list_id}')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_featured_episode_menu(self):
        """Featured episodes menu: /featured/<cfeatured>"""
        episode_list_id = '$byUzNXxwJS98bGlzdF8yMDcyNjA4OTgwfGIlMHxuJTEl'
        addon.run([f'{PLUGIN_BASE_URL}/featured/episode_{episode_list_id}', '0', ''])
        self.assertEqual(plugin.url_for(addon.featured,
                                        feature=f'episode_{episode_list_id}'),
                         f'{PLUGIN_BASE_URL}/featured/episode_{episode_list_id}')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_channels_menu(self):
        """Channels menu = /channels/<channel>"""
        addon.run([f'{PLUGIN_BASE_URL}/channels', '0', ''])
        self.assertEqual(plugin.url_for(addon.channels), f'{PLUGIN_BASE_URL}/channels')
        addon.run([f'{PLUGIN_BASE_URL}/channels/ketnet', '0', ''])
        self.assertEqual(plugin.url_for(addon.channels, channel='ketnet'), f'{PLUGIN_BASE_URL}/channels/ketnet')

    def test_livetv_menu(self):
        """Live TV menu: /livetv"""
        addon.run([f'{PLUGIN_BASE_URL}/livetv', '0', ''])
        self.assertEqual(plugin.url_for(addon.livetv), f'{PLUGIN_BASE_URL}/livetv')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_recent_menu(self):
        """Most recent menu: /recent"""
        addon.run([f'{PLUGIN_BASE_URL}/recent', '0', ''])
        self.assertEqual(plugin.url_for(addon.recent), f'{PLUGIN_BASE_URL}/recent')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_offline_menu(self):
        """Soon offline menu: /offline"""
        addon.run([f'{PLUGIN_BASE_URL}/offline', '0', ''])
        self.assertEqual(plugin.url_for(addon.offline), f'{PLUGIN_BASE_URL}/offline')

    def test_tvguide_date_menu(self):
        """TV guide menu: /tvguide/<date>/<channel>"""
        addon.run([f'{PLUGIN_BASE_URL}/tvguide', '0', ''])
        self.assertEqual(plugin.url_for(addon.tvguide), f'{PLUGIN_BASE_URL}/tvguide/date')
        addon.run([f'{PLUGIN_BASE_URL}/tvguide/date/today', '0', ''])
        self.assertEqual(plugin.url_for(addon.tvguide, date='today'), f'{PLUGIN_BASE_URL}/tvguide/date/today')
        addon.run([f'{PLUGIN_BASE_URL}/tvguide/date/today/canvas', '0', ''])
        self.assertEqual(plugin.url_for(addon.tvguide, date='today', channel='canvas'), f'{PLUGIN_BASE_URL}/tvguide/date/today/canvas')
        addon.run([f'{PLUGIN_BASE_URL}/tvguide/channel/canvas', '0', ''])
        self.assertEqual(plugin.url_for(addon.tvguide_channel, channel='canvas'), f'{PLUGIN_BASE_URL}/tvguide/channel/canvas')
        addon.run([f'{PLUGIN_BASE_URL}/tvguide/channel/canvas/today', '0', ''])
        self.assertEqual(plugin.url_for(addon.tvguide_channel, channel='canvas', date='today'),
                         f'{PLUGIN_BASE_URL}/tvguide/channel/canvas/today')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_search_history(self):
        """Add search keyword: /search/add/<keywords>
            Clear search history: /search/clear
            Remove search keyword: /search/remove/<keywords>"""
        addon.run([f'{PLUGIN_BASE_URL}/search/add/foobar', '0', ''])
        self.assertEqual(plugin.url_for(addon.add_search, keywords='foobar'), f'{PLUGIN_BASE_URL}/search/add/foobar')
        addon.run([f'{PLUGIN_BASE_URL}/search/add/foobar', '0', ''])
        self.assertEqual(plugin.url_for(addon.add_search, keywords='foobar'), f'{PLUGIN_BASE_URL}/search/add/foobar')
        addon.run([f'{PLUGIN_BASE_URL}/search/query/foobar', '0', ''])
        self.assertEqual(plugin.url_for(addon.add_search, keywords='foobar'), f'{PLUGIN_BASE_URL}/search/add/foobar')
        addon.run([f'{PLUGIN_BASE_URL}/search/edit', '0', ''])
        self.assertEqual(plugin.url_for(addon.edit_search), f'{PLUGIN_BASE_URL}/search/edit')
        addon.run([f'{PLUGIN_BASE_URL}/search/edit/foobar', '0', ''])
        self.assertEqual(plugin.url_for(addon.edit_search, keywords='foobar'), f'{PLUGIN_BASE_URL}/search/edit/foobar')
        addon.run([f'{PLUGIN_BASE_URL}/search/remove/foobar', '0', ''])
        self.assertEqual(plugin.url_for(addon.remove_search, keywords='foobar'), f'{PLUGIN_BASE_URL}/search/remove/foobar')
        addon.run([f'{PLUGIN_BASE_URL}/search/remove/foobar', '0', ''])
        self.assertEqual(plugin.url_for(addon.remove_search, keywords='foobar'), f'{PLUGIN_BASE_URL}/search/remove/foobar')
        addon.run([f'{PLUGIN_BASE_URL}/search/clear', '0', ''])
        self.assertEqual(plugin.url_for(addon.clear_search), f'{PLUGIN_BASE_URL}/search/clear')
        addon.run([f'{PLUGIN_BASE_URL}/search', '0', ''])
        self.assertEqual(plugin.url_for(addon.search), f'{PLUGIN_BASE_URL}/search')
        addon.run([f'{PLUGIN_BASE_URL}/search/add/foobar', '0', ''])
        self.assertEqual(plugin.url_for(addon.add_search, keywords='foobar'), f'{PLUGIN_BASE_URL}/search/add/foobar')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_search_menu(self):
        """Search VRT MAX menu: /search/query/<keywords>"""
        addon.run([f'{PLUGIN_BASE_URL}/search', '0', ''])
        self.assertEqual(plugin.url_for(addon.search), f'{PLUGIN_BASE_URL}/search')
        addon.run([f'{PLUGIN_BASE_URL}/search/query', '0', ''])
        self.assertEqual(plugin.url_for(addon.search_query), f'{PLUGIN_BASE_URL}/search/query')
        addon.run([f'{PLUGIN_BASE_URL}/search/query/dag', '0', ''])
        self.assertEqual(plugin.url_for(addon.search_query, keywords='dag'), f'{PLUGIN_BASE_URL}/search/query/dag')
        addon.run([f'{PLUGIN_BASE_URL}/search/query/winter', '0', ''])
        self.assertEqual(plugin.url_for(addon.search_query, keywords='winter'), f'{PLUGIN_BASE_URL}/search/query/winter')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_follow_route(self):
        """Follow method: /follow/<program_id>/<program_title>"""
        addon.run([f'{PLUGIN_BASE_URL}/follow/1459955889901/Thuis', '0', ''])
        self.assertEqual(plugin.url_for(addon.follow, program_id='1459955889901', program_title='Thuis'),
                         f'{PLUGIN_BASE_URL}/follow/1459955889901/Thuis')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_unfollow_route(self):
        """Unfollow method: /unfollow/<program_id>/<program_title>"""
        addon.run([f'{PLUGIN_BASE_URL}/unfollow/1459955889901/Thuis', '0', ''])
        self.assertEqual(plugin.url_for(addon.unfollow, program_id='1459955889901', program_title='Thuis'),
                         f'{PLUGIN_BASE_URL}/unfollow/1459955889901/Thuis')

    def test_clear_cookies_route(self):
        """Delete tokens method: /tokens/delete"""
        addon.run([f'{PLUGIN_BASE_URL}/tokens/delete', '0', ''])
        self.assertEqual(plugin.url_for(addon.delete_tokens), f'{PLUGIN_BASE_URL}/tokens/delete')

    def test_invalidate_caches_route(self):
        """Delete cache method: /cache/delete"""
        addon.run([f'{PLUGIN_BASE_URL}/cache/delete', '0', ''])
        self.assertEqual(plugin.url_for(addon.delete_cache), f'{PLUGIN_BASE_URL}/cache/delete')

    def test_play_on_demand_by_id_route(self):
        """Play on demand by id: /play/id/<publication_id>/<video_id>"""
        # Achterflap episode 8 available until 31/12/2025
        addon.run([f'{PLUGIN_BASE_URL}/play/id/vid-f80fa527-6759-45a7-908d-ec6f0a7b164e/pbs-pub-1a170972-dea3-4ea3-8c27-62d2442ee8a3', '0', ''])
        self.assertEqual(plugin.url_for(addon.play_id,
                                        video_id='vid-f80fa527-6759-45a7-908d-ec6f0a7b164e',
                                        publication_id='pbs-pub-1a170972-dea3-4ea3-8c27-62d2442ee8a3'),
                         f'{PLUGIN_BASE_URL}/play/id/vid-f80fa527-6759-45a7-908d-ec6f0a7b164e/pbs-pub-1a170972-dea3-4ea3-8c27-62d2442ee8a3')

    def test_play_livestream_by_id_route(self):
        """Play livestream by id: /play/id/<video_id>"""
        # Canvas livestream
        addon.run([f'{PLUGIN_BASE_URL}/play/id/vualto_canvas_geo', '0', ''])
        self.assertEqual(plugin.url_for(addon.play_id, video_id='vualto_canvas_geo'), f'{PLUGIN_BASE_URL}/play/id/vualto_canvas_geo')

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_play_latestepisode_route(self):
        """Play last episode method: /play/lastepisode/<program>"""
        addon.run([f'{PLUGIN_BASE_URL}/play/latest/vrt-nws-journaal', '0', ''])
        self.assertEqual(plugin.url_for(addon.play_latest, program_name='vrt-nws-journaal'), f'{PLUGIN_BASE_URL}/play/latest/vrt-nws-journaal')
        addon.run([f'{PLUGIN_BASE_URL}/play/latest/terzake', '0', ''])
        self.assertEqual(plugin.url_for(addon.play_latest, program_name='terzake'), f'{PLUGIN_BASE_URL}/play/latest/terzake')
        addon.run([f'{PLUGIN_BASE_URL}/play/latest/winteruur', '0', ''])
        self.assertEqual(plugin.url_for(addon.play_latest, program_name='winteruur'), f'{PLUGIN_BASE_URL}/play/latest/winteruur')

    def test_play_airdateepisode_route(self):
        """Play episode by air date method: /play/airdate/<channel>/<start_date>/<end_date>"""
        # Test Het Journaal
        addon.run([lastweek.strftime(f'{PLUGIN_BASE_URL}/play/airdate/een/%Y-%m-%dT19:00:00/%Y-%m-%dT19:45:00'), '0', ''])
        self.assertEqual(plugin.url_for(addon.play_air_date,
                                        channel='een',
                                        start_date=lastweek.strftime('%Y-%m-%dT19:00:00/%Y-%m-%dT19:45:00')),
                         lastweek.strftime(f'{PLUGIN_BASE_URL}/play/airdate/een/%Y-%m-%dT19:00:00/%Y-%m-%dT19:45:00'))
        # Test TerZake
        addon.run([lastweek.strftime(f'{PLUGIN_BASE_URL}/play/airdate/canvas/%Y-%m-%dT20:00:00/%Y-%m-%dT20:35:00'), '0', ''])
        self.assertEqual(plugin.url_for(addon.play_air_date,
                                        channel='canvas',
                                        start_date=lastweek.strftime('%Y-%m-%dT20:00:00/%Y-%m-%dT20:35:00')),
                         lastweek.strftime(f'{PLUGIN_BASE_URL}/play/airdate/canvas/%Y-%m-%dT20:00:00/%Y-%m-%dT20:35:00'))
        # Test Livestream cache for morning tv from 9h to 10h
        if now.hour < 10:
            mostrecent = now + timedelta(days=-1)
        else:
            mostrecent = now
        addon.run([mostrecent.strftime(f'{PLUGIN_BASE_URL}/play/airdate/een/%Y-%m-%dT09:00:00/%Y-%m-%dT10:00:00'), '0', ''])
        self.assertEqual(plugin.url_for(addon.play_air_date,
                                        channel='een',
                                        start_date=mostrecent.strftime('%Y-%m-%dT09:00:00'),
                                        end_date=mostrecent.strftime('%Y-%m-%dT10:00:00')),
                         mostrecent.strftime(f'{PLUGIN_BASE_URL}/play/airdate/een/%Y-%m-%dT09:00:00/%Y-%m-%dT10:00:00'))

    @unittest.skipUnless(xbmc_addon.settings.get('username'), 'Skipping as VRT username is missing.')
    @unittest.skipUnless(xbmc_addon.settings.get('password'), 'Skipping as VRT password is missing.')
    def test_play_upnext_route(self):
        """Play Up Next episode method: /play/upnext/<episode_id>"""
        addon.run([f'{PLUGIN_BASE_URL}/play/upnext//vrtmax/a-z/roomies/2/roomies-s2a2/', '0', ''])
        self.assertEqual(plugin.url_for(addon.play_upnext, episode_id='/vrtmax/a-z/roomies/2/roomies-s2a2/'),
                         f'{PLUGIN_BASE_URL}/play/upnext//vrtmax/a-z/roomies/2/roomies-s2a2/')

    def test_update_repos(self):
        """Update repositories: /update/repos"""
        addon.run([f'{PLUGIN_BASE_URL}/update/repos', '0', ''])
        self.assertEqual(plugin.url_for(addon.update_repos), f'{PLUGIN_BASE_URL}/update/repos')

    def test_show_settings_addons(self):
        """Open the Kodi System settings: /show/settings/addons"""
        addon.run([f'{PLUGIN_BASE_URL}/show/settings/addons', '0', ''])
        self.assertEqual(plugin.url_for(addon.show_settings_addons), f'{PLUGIN_BASE_URL}/show/settings/addons')


if __name__ == '__main__':
    unittest.main()
