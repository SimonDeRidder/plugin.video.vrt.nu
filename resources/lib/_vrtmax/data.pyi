from typing import TypedDict

class _Category(TypedDict):
    id: str
    name: str
    msgctxt: int

class _YoutubeChannel(TypedDict):
    label: str
    url: str

class _Channel(TypedDict):
    id: str
    name: str
    label: str
    studio: str
    live_stream: str | None
    live_stream_id: str | None
    youtube: list[_YoutubeChannel]
    has_tvguide: bool
    logo: str | None
    epg_id: str | None
    preset: int | None
    vod: bool

class _RelativeDate(TypedDict):
    id: str
    offset: int
    msgctxt: int
    permalink: bool


def categories() -> list[_Category]: ...
def channels() -> list[_Channel]: ...
def relative_dates() -> list[_RelativeDate]: ...
