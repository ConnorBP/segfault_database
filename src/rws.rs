// some functions for calculating RWS from round points

/*
 * This isn't meant to be a comprehensive stats system, it's meant to be a simple
 * way to balance teams to replace manual stuff using a (exponentially) weighted moving average.
 * The update takes place every round, following this equation
 *
 * R' = (1-a) * R_prev + alpha * R
 * Where
 *    R' is the new rating
 *    a is the alpha factor (how much a new round counts into the new rating)
 *    R is the round-rating
 *
 * Alpha is made to be variable, where it decreases linearly to allow
 * ratings to change more quickly early on when a player has few rounds played.
 *
#define ALPHA_INIT 0.1
#define ALPHA_FINAL 0.003
#define ROUNDS_FINAL 250.0
*/

//  how fast should the average move at zero rounds played
static ALPHA_INIT: f32 = 0.1;
//  final ammount of changing after rounds final reached
static ALPHA_FINAL: f32 = 0.003;
// How many rounds will averages change fast for
static ROUNDS_FINAL: f32 = 250.0;

/*
Event_RoundEnd() {
    int winner = event.GetInt("winner");
    for (int i = 1; i <= MaxClients; i++) {
        if (IsPlayer(i) && IsOnDb(i)) {
            int team = GetClientTeam(i);
            if (team == CS_TEAM_CT || team == CS_TEAM_T){RWSUpdate(i, team == winner);}
        }
    }
    for (int i = 1; i <= MaxClients; i++) {
        if (IsPlayer(i) && IsOnDb(i)) {
            g_aStats[i].ROUND_POINTS = 0;
            SavePlayerData(i);
        }
    }
}

/**
 * Here we apply magic updates to a player's rws based on the previous round.
 */
static void RWSUpdate(int client, bool winner) {
  float rws = 0.0;
  if (winner) {
    int playerCount = 0;
    int sum = 0;
    for (int i = 1; i <= MaxClients; i++) {
      if (IsPlayer(i)) {
        if (GetClientTeam(i) == GetClientTeam(client)) {
          sum += g_aStats[i].ROUND_POINTS;
          playerCount++;
        }
      }
    }

    if (sum != 0) {
      // scaled so it's always considered "out of 5 players" so different team sizes
      // don't give inflated rws
      rws = 100.0 * float(playerCount) / 5.0 * float(g_aStats[client].ROUND_POINTS) / float(sum);
    } else {
      return;
    }

  } else {
    rws = 0.0;
  }

  float alpha = GetAlphaFactor(client);
  g_aStats[client].RWS = (1.0 - alpha) * g_aStats[client].RWS + alpha * rws;
  g_aStats[client].ROUNDS_TOTAL++;
  LogDebug("RoundUpdate(%L), alpha=%f, round_rws=%f, new_rws=%f", client, alpha, rws,
           g_aStats[client].RWS);
}



// some utils (TODO MOVE THIS TO UTIL FILE)

public bool IsOnDb(int client) {
  return OnDB[client];
}

// Re-Usable checks for wether or not we should rank players right now
bool ShouldRank() {
    // ranks should be calculated if it is not warmup, and there are at least the min player count (2 by default)
    // TODO: add check for if ranking is by round or by match either here or somewhere else
    return !CheckIfWarmup() && g_MinimumPlayers > GetCurrentPlayers();
}

// returns true if it is currently the warmup period
bool CheckIfWarmup() {
    return GameRules_GetProp("m_bWarmupPeriod") == 1;
}


static float GetAlphaFactor(int client) {
  float rounds = float(g_aStats[client].ROUNDS_TOTAL);
  if (rounds < ROUNDS_FINAL) {
    return ALPHA_INIT + (ALPHA_INIT - ALPHA_FINAL) / (-ROUNDS_FINAL) * rounds;
  } else {
    return ALPHA_FINAL;
  }
}

public int rwsSortFunction(int index1, int index2, Handle array, Handle hndl) {
  int client1 = GetArrayCell(array, index1);
  int client2 = GetArrayCell(array, index2);
  return g_aStats[client1].RWS < g_aStats[client2].RWS;
}*/

// this assumes there are enough players in the game (non zero) to actually reward the player with score
pub fn calculate_rws(
    oldRws: f32,
    totalRounds: f32,
    wonRound: bool,
    roundPoints: f32,
    teamPoints: f32,
    teamPlayerCount: f32,
) -> f32 {
    let mut roundRws = 0.0;

    // this is required in the plugin to get the team points and playercount:
    /*let teamPlayerCount = 0;
    let sum = 0;
    for (int i = 1; i <= MaxClients; i++) {
      if (IsPlayer(i)) {
        if (GetClientTeam(i) == GetClientTeam(client)) {
          sum += g_aStats[i].ROUND_POINTS;
          teamPlayerCount++;
        }
      }
    }*/

    if (teamPoints != 0.0 && roundPoints != 0.0 && teamPlayerCount >= 1.0) {
        // scaled so it's always considered "out of 5 players" so different team sizes
        // don't give inflated rws
        roundRws = 100.0 * teamPlayerCount / 5.0 * roundPoints / teamPoints;
    } else {
        return oldRws;
    }
    if (!wonRound) {
        // if they didn't win, give them a quarter of their contribution points instead of nothing
        roundRws = roundRws * 0.25;
    }

    let alpha = GetAlphaFactor(totalRounds);

    //let newRws = (1.0 - alpha) * oldRws + alpha * roundRws;
    //newRws
    // Calculate the new rws average using the alpha factor to speed up changes at first

    //println!("getting rws with alpha: {} old: {} rounds: {} won: {} roundpoints: {} teampoints: {} players: {}\nnew value: {} round rws: {}", alpha, oldRws, totalRounds, wonRound, roundPoints, teamPoints, teamPlayerCount, (1.0 - alpha) * oldRws + alpha * roundRws, roundRws);

    (1.0 - alpha) * oldRws + alpha * roundRws
    // now increment the total rounds after this function, not before
}

fn GetAlphaFactor(rounds: f32) -> f32 {
    if (rounds < ROUNDS_FINAL) {
        return ALPHA_INIT + (ALPHA_INIT - ALPHA_FINAL) / (-ROUNDS_FINAL) * rounds;
    } else {
        return ALPHA_FINAL;
    }
}
