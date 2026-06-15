# -*- coding: utf-8 -*-
# GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
"""This is the actual VRT MAX service entry point"""
from native_init import init_vrtmax
from service import VrtMonitor

init_vrtmax()

VrtMonitor().run()
