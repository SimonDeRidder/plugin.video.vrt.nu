# -*- coding: utf-8 -*-
# GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
"""Build the settings dict for _vrtmax.init / _vrtmax.update_settings."""

from kodiutils import (
    addon_profile,
    can_play_drm,
    get_addon_info,
    get_cache_dir,
    get_max_bandwidth,
    get_setting,
    get_setting_bool,
    get_setting_int,
    has_addon,
    has_credentials,
    has_inputstream_adaptive,
    kodi_version_major,
    log,
    supports_drm,
    translate_path,
)


def build_settings_dict() -> dict[str, str | bool | int]:
    """Snapshot the full Settings shape Rust expects."""
    return {
        # User prefs — defaults match kodiutils call sites.
        'username': get_setting('username', default=''),
        'password': get_setting('password', default=''),
        'itemsperpage': get_setting_int('itemsperpage', default=50),
        'usefavorites': get_setting_bool('usefavorites', default=True),
        'useresumepoints': get_setting_bool('useresumepoints', default=True),
        'showpermalink': get_setting_bool('showpermalink', default=False),
        'showfanart': get_setting_bool('showfanart', default=True),
        'showyoutube': get_setting_bool('showyoutube', default=True),
        'usedrm': get_setting_bool('usedrm', default=True),
        'useinputstreamadaptive': get_setting_bool('useinputstreamadaptive', default=True),
        'max_bandwidth': get_max_bandwidth(),
        # Kodi-env state
        'kodi_version_major': kodi_version_major(),
        'has_inputstream_adaptive': has_inputstream_adaptive(),
        'can_play_drm': can_play_drm(),
        'supports_drm': supports_drm(),
        'has_credentials': has_credentials(),
        'has_studios_white': has_addon('resource.images.studios.white'),
        'has_youtube': has_addon('plugin.video.youtube'),
        'has_iptv_manager': has_addon('service.iptv.manager'),
    }


def init_vrtmax() -> None:
    """One-shot per-process init."""
    import _vrtmax
    addon_ver = get_addon_info('version')
    native_ver = getattr(_vrtmax, '__version__', '?')
    if native_ver != addon_ver:
        log(
            1,
            'VRT: native module version {} != addon {}. Fully restart Kodi to load new version.'.format(native_ver, addon_ver)
        )

    cache_dir = translate_path(get_cache_dir())
    profile_dir = addon_profile()  # already translated

    def _log_cb(level: int, message: str) -> None:
        log(level, message)

    _vrtmax.init(build_settings_dict(), cache_dir, profile_dir, _log_cb)


def push_settings() -> None:
    """Refresh Rust's Settings snapshot.
    """
    import _vrtmax

    _vrtmax.update_settings(build_settings_dict())
