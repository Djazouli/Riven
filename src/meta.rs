///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version f60af07c98f05dffdaf81262f9b01f97fe94a3a1

//! Metadata about the Riot API and Riven.
//!
//! Note: this modules is automatically generated.

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENDPOINT_PATH_METHODID: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("/riot/account/v1/accounts/by-puuid/{puuid}", "account-v1.getByPuuid");
        map.insert("/riot/account/v1/accounts/by-riot-id/{gameName}/{tagLine}", "account-v1.getByRiotId");
        map.insert("/riot/account/v1/active-shards/by-game/{game}/by-puuid/{puuid}", "account-v1.getActiveShard");
        map.insert("/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}", "champion-mastery-v4.getAllChampionMasteries");
        map.insert("/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}/by-champion/{championId}", "champion-mastery-v4.getChampionMastery");
        map.insert("/lol/champion-mastery/v4/scores/by-summoner/{encryptedSummonerId}", "champion-mastery-v4.getChampionMasteryScore");
        map.insert("/lol/platform/v3/champion-rotations", "champion-v3.getChampionInfo");
        map.insert("/lol/clash/v1/players/by-summoner/{summonerId}", "clash-v1.getPlayersBySummoner");
        map.insert("/lol/clash/v1/teams/{teamId}", "clash-v1.getTeamById");
        map.insert("/lol/clash/v1/tournaments", "clash-v1.getTournaments");
        map.insert("/lol/clash/v1/tournaments/by-team/{teamId}", "clash-v1.getTournamentByTeam");
        map.insert("/lol/clash/v1/tournaments/{tournamentId}", "clash-v1.getTournamentById");
        map.insert("/lol/league-exp/v4/entries/{queue}/{tier}/{division}", "league-exp-v4.getLeagueEntries");
        map.insert("/lol/league/v4/challengerleagues/by-queue/{queue}", "league-v4.getChallengerLeague");
        map.insert("/lol/league/v4/entries/by-summoner/{encryptedSummonerId}", "league-v4.getLeagueEntriesForSummoner");
        map.insert("/lol/league/v4/entries/{queue}/{tier}/{division}", "league-v4.getLeagueEntries");
        map.insert("/lol/league/v4/grandmasterleagues/by-queue/{queue}", "league-v4.getGrandmasterLeague");
        map.insert("/lol/league/v4/leagues/{leagueId}", "league-v4.getLeagueById");
        map.insert("/lol/league/v4/masterleagues/by-queue/{queue}", "league-v4.getMasterLeague");
        map.insert("/lol/status/v3/shard-data", "lol-status-v3.getShardData");
        map.insert("/lol/status/v4/platform-data", "lol-status-v4.getPlatformData");
        map.insert("/lor/match/v1/matches/by-puuid/{puuid}/ids", "lor-match-v1.getMatchIdsByPUUID");
        map.insert("/lor/match/v1/matches/{matchId}", "lor-match-v1.getMatch");
        map.insert("/lor/ranked/v1/leaderboards", "lor-ranked-v1.getLeaderboards");
        map.insert("/lor/status/v1/platform-data", "lor-status-v1.getPlatformData");
        map.insert("/lol/match/v4/matches/by-tournament-code/{tournamentCode}/ids", "match-v4.getMatchIdsByTournamentCode");
        map.insert("/lol/match/v4/matches/{matchId}", "match-v4.getMatch");
        map.insert("/lol/match/v4/matches/{matchId}/by-tournament-code/{tournamentCode}", "match-v4.getMatchByTournamentCode");
        map.insert("/lol/match/v4/matchlists/by-account/{encryptedAccountId}", "match-v4.getMatchlist");
        map.insert("/lol/match/v4/timelines/by-match/{matchId}", "match-v4.getMatchTimeline");
        map.insert("/lol/match/v5/matches/by-puuid/{puuid}/ids", "match-v5.getMatchIdsByPUUID");
        map.insert("/lol/match/v5/matches/{matchId}", "match-v5.getMatch");
        map.insert("/lol/match/v5/matches/{matchId}/timeline", "match-v5.getTimeline");
        map.insert("/lol/spectator/v4/active-games/by-summoner/{encryptedSummonerId}", "spectator-v4.getCurrentGameInfoBySummoner");
        map.insert("/lol/spectator/v4/featured-games", "spectator-v4.getFeaturedGames");
        map.insert("/lol/summoner/v4/summoners/by-account/{encryptedAccountId}", "summoner-v4.getByAccountId");
        map.insert("/lol/summoner/v4/summoners/by-name/{summonerName}", "summoner-v4.getBySummonerName");
        map.insert("/lol/summoner/v4/summoners/by-puuid/{encryptedPUUID}", "summoner-v4.getByPUUID");
        map.insert("/lol/summoner/v4/summoners/{encryptedSummonerId}", "summoner-v4.getBySummonerId");
        map.insert("/tft/league/v1/challenger", "tft-league-v1.getChallengerLeague");
        map.insert("/tft/league/v1/entries/by-summoner/{summonerId}", "tft-league-v1.getLeagueEntriesForSummoner");
        map.insert("/tft/league/v1/entries/{tier}/{division}", "tft-league-v1.getLeagueEntries");
        map.insert("/tft/league/v1/grandmaster", "tft-league-v1.getGrandmasterLeague");
        map.insert("/tft/league/v1/leagues/{leagueId}", "tft-league-v1.getLeagueById");
        map.insert("/tft/league/v1/master", "tft-league-v1.getMasterLeague");
        map.insert("/tft/league/v1/rated-ladders/{queue}/top", "tft-league-v1.getTopRatedLadder");
        map.insert("/tft/match/v1/matches/by-puuid/{puuid}/ids", "tft-match-v1.getMatchIdsByPUUID");
        map.insert("/tft/match/v1/matches/{matchId}", "tft-match-v1.getMatch");
        map.insert("/tft/summoner/v1/summoners/by-account/{encryptedAccountId}", "tft-summoner-v1.getByAccountId");
        map.insert("/tft/summoner/v1/summoners/by-name/{summonerName}", "tft-summoner-v1.getBySummonerName");
        map.insert("/tft/summoner/v1/summoners/by-puuid/{encryptedPUUID}", "tft-summoner-v1.getByPUUID");
        map.insert("/tft/summoner/v1/summoners/{encryptedSummonerId}", "tft-summoner-v1.getBySummonerId");
        map.insert("/lol/platform/v4/third-party-code/by-summoner/{encryptedSummonerId}", "third-party-code-v4.getThirdPartyCodeBySummonerId");
        map.insert("/lol/tournament-stub/v4/lobby-events/by-code/{tournamentCode}", "tournament-stub-v4.getLobbyEventsByCode");
        map.insert("/lol/tournament/v4/codes/{tournamentCode}", "tournament-v4.getTournamentCode");
        map.insert("/lol/tournament/v4/lobby-events/by-code/{tournamentCode}", "tournament-v4.getLobbyEventsByCode");
        map.insert("/val/content/v1/contents", "val-content-v1.getContent");
        map.insert("/val/match/v1/matches/{matchId}", "val-match-v1.getMatch");
        map.insert("/val/match/v1/matchlists/by-puuid/{puuid}", "val-match-v1.getMatchlist");
        map.insert("/val/match/v1/recent-matches/by-queue/{queue}", "val-match-v1.getRecent");
        map.insert("/val/ranked/v1/leaderboards/by-act/{actId}", "val-ranked-v1.getLeaderboard");
        map.insert("/val/status/v1/platform-data", "val-status-v1.getPlatformData");
        map
    };
}
