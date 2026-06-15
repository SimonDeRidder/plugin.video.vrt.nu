# -*- coding: utf-8 -*-
# GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
"""Implements static functions used elsewhere in the add-on"""

import re
from datetime import timedelta

import _vrtmax.utils as _utils

reformat_image_url = _utils.reformat_image_url
url_to_program = _utils.url_to_program
play_url_to_id = _utils.play_url_to_id
shorten_link = _utils.shorten_link
find_entry = _utils.find_entry
youtube_to_plugin_url = _utils.youtube_to_plugin_url


ISO_DURATION = re.compile(
    r'^P'                                     # starts with P
    r'(?:(?P<days>\d+(?:\.\d+)?)D)?'          # days (with optional decimals)
    r'(?:T'                                   # time part
    r'(?:(?P<hours>\d+(?:\.\d+)?)H)?'         # hours
    r'(?:(?P<minutes>\d+(?:\.\d+)?)M)?'       # minutes
    r'(?:(?P<seconds>\d+(?:\.\d+)?)S)?'       # seconds
    r')?$'
)


def parse_duration(s: str) -> timedelta:
    """
    Parse an ISO 8601 duration string (days, hours, minutes, seconds)
    into a datetime.timedelta. Supports fractional values.
    Does not support months or years.
    """
    match = ISO_DURATION.match(s)
    if not match:
        raise ValueError(f"Invalid ISO 8601 duration: {s}")
    parts = {k: float(v) if v else 0.0 for k, v in match.groupdict().items()}
    return timedelta(days=parts['days'],
                     hours=parts['hours'],
                     minutes=parts['minutes'],
                     seconds=parts['seconds'])
