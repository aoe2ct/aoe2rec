from collections import defaultdict
import collections
from dataclasses import dataclass
from datetime import datetime, timedelta
import json
from typing import BinaryIO

try:
    from typing import override
except ImportError:

    def override(f):
        return f


from aoe2rec_py import aoe2rec_py


@dataclass
class Chat:
    timestamp: timedelta
    player: str
    message: str

    @override
    def __str__(self):
        return f"{self.timestamp} - {self.player}: {self.message}"


class RecSummary:
    def __init__(self, handle: BinaryIO):
        data = handle.read()
        self._cache = aoe2rec_py.parse_rec(data)
        self.players = {
            player_id + 1: {"resigned": False, "elo": 0, "eapm": 0, **player}
            for player_id, player in enumerate(
                self._cache["zheader"]["game_settings"]["players"]
            )
        }
        self.duration: float = 0
        self.chats: list[Chat] = []

        self._parse_operations()

    def _parse_operations(self):
        eapm_counter = collections.Counter()
        for event in self._cache["operations"]:
            if "Sync" in event:
                self.duration += event["Sync"]["time_increment"]
            if "Chat" in event:
                chat = json.loads(event["Chat"]["text"])
                self.chats.append(
                    Chat(
                        self.get_duration(),
                        self.players[chat["player"]]["name"],
                        chat["message"],
                    )
                )
            if "Action" in event:
                actions = event["Action"]["action_data"]
                for action_type, action_data in actions.items():
                    if "player_id" in action_data:
                        eapm_counter[action_data["player_id"]] += 1
                        if action_type == "Resign":
                            self.players[action_data["player_id"]]["resigned"] = True
            if "PostGame" in event:
                for block in event["PostGame"]["blocks"]:
                    if (
                        "Leaderboards" not in block
                        or block["Leaderboards"]["num_leaderboards"] < 1
                    ):
                        continue
                    for player in block["Leaderboards"]["leaderboards"][0]["players"]:
                        if (player["player_number"] + 1) in self.players:
                            self.players[player["player_number"] + 1]["elo"] = player[
                                "elo"
                            ]
        total_minutes = self.get_duration().total_seconds() / 60
        for player_id, action_count in eapm_counter.items():
            self.players[player_id]["eapm"] = int(round(action_count / total_minutes))

    def get_chat(self):
        return self.chats

    def get_postgame(self):
        return None

    def has_achievements(self):
        return False

    def get_header(self):
        return self._cache["zheader"]

    def get_start_time(self):
        return self._cache["zheader"]["replay"]["world_time"]

    def get_duration(self):
        return timedelta(
            milliseconds=self.duration + self._cache["zheader"]["replay"]["world_time"]
        )

    def get_restored(self):
        return self.get_start_time() > 0, self.get_start_time()

    def get_version(self):
        header = self._cache["zheader"]
        major = header["version_major"]
        minor = header["version_minor"]
        version = float(f"{major}.{minor}")
        return (
            "DE",
            header["game"],
            version,
            self._cache["log_version"],
            header["build"],
        )

    def get_owner(self):
        return self._cache["zheader"]["replay"]["rec_player"]

    def get_teams(self):
        teams: defaultdict[int, list[int]] = defaultdict(list)
        for player_id, player in self.players.items():
            team_id: int = player["resolved_team_id"]
            if team_id > 1:
                teams[team_id].append(player_id)
            elif team_id == 1:
                teams[player_id + 8].append(player_id)
        return set([frozenset(s) for s in teams.values()])

    def get_diplomacy(self):
        diplo_type = self._get_diplomacy_type()
        if diplo_type == "FFA":
            return {"type": diplo_type, "team_size": "FFA"}
        team_sizes = [str(len(team)) for team in self.get_teams()]
        return {"type": diplo_type, "team_size": "v".join(team_sizes)}

    def get_players(self):
        return [
            {
                "name": player["name"],
                "number": player_id,
                "civilization": player["civ_id"],
                "color_id": player["color_id"],
                "human": player["player_type"] == 2,
                "winner": not player["resigned"],
                "user_id": player["profile_id"],
                "position": [
                    None,
                    None,
                ],  # TODO: Parse players objects and find starting TC
                "rate_snapshot": player["elo"],
                "prefer_random": player["prefer_random"],
                "eapm": player["eapm"],
            }
            for player_id, player in self.players.items()
        ]

    def get_objects(self):
        raise NotImplementedError()

    def get_platform(self):
        settings = self._cache["zheader"]["game_settings"]
        guid = settings["guid"]
        guid_str = f"{guid[0]:02x}{guid[1]:02x}{guid[2]:02x}{guid[3]:02x}-{guid[4]:02x}{guid[5]:02x}-{guid[6]:02x}{guid[7]:02x}-{guid[8]:02x}{guid[9]:02x}-{guid[10]:02x}{guid[11]:02x}{guid[12]:02x}{guid[13]:02x}{guid[14]:02x}{guid[15]:02x}"
        return {
            "platform_id": "de",
            "platform_match_id": guid_str,
            "rated": settings["ranked"],
            "lobby_name": settings["lobby_name"],
            "spec_delay": settings["spec_delay"],
            "allow_specs": settings["allow_specs"],
            "private": settings["lobby_visibility"] == 2,
        }

    def _get_diplomacy_type(self):
        n_teams = len(self.get_teams())
        n_players = len(self.players)
        if n_teams == 2 and n_players > 2:
            return "TG"
        if n_players == 2:
            return "1v1"
        if n_teams == n_players or (n_teams == 1 and n_players > 2):
            return "FFA"
        return "Other"

    def get_settings(self):
        settings = self._cache["zheader"]["game_settings"]
        # TODO: Add missing names from constants in aocref
        return {
            "type": (settings["game_type"], "<Missing>"),
            "difficulty": (settings["difficulty"], "<Missing>"),
            "population_limit": settings["population_limit"],
            "speed": (settings["speed"], "<Missing>"),
            "cheats": settings["cheats"],
            "team_together": settings["team_positions"],
            "all_technologies": settings["all_techs"],
            "lock_speed": settings["lock_speed"],
            "lock_teams": settings["lock_teams"],
            "map_reveal_choice": (settings["reveal_map"], "<Missing>"),
            "diplomacy_type": self._get_diplomacy_type(),
            "starting_resouces": (settings["starting_resources_id"], "<Missing>"),
            "starting_age": (settings["starting_age_id"], "<Missing>"),
            "ending_age": (settings["ending_age_id"], "<Missing>"),
            "victory_condition": (settings["victory_type_id"], "<Missing>"),
            "treaty_length": settings["treaty_length"],
            "multiqueue": True,  # Always true for DE
            "hidden_civs": settings["hidden_civs"],
        }

    def get_file_hash(self):
        raise NotImplementedError()

    def get_hash(self):
        raise NotImplementedError()

    def get_encoding(self):
        raise NotImplementedError()

    def get_language(self):
        raise NotImplementedError()

    def get_device(self):
        raise NotImplementedError()

    def get_map(self):
        raise NotImplementedError()

    def get_dataset(self):
        raise NotImplementedError()

    def get_completed(self):
        raise NotImplementedError()

    def get_mirror(self):
        raise NotImplementedError()

    def get_played(self):
        return datetime.fromtimestamp(
            self._cache["zheader"]["game_settings"]["timestamp"]
        )


class NotImplementedError(Exception):
    pass


def test():
    with open("ClickBait_vs_Numerfolt_G1b.aoe2record", "rb") as f:
        return RecSummary(f)
