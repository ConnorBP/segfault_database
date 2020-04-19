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
static ALPHA_INIT: f32 = 0.2;
//  final ammount of changing after rounds final reached
static ALPHA_FINAL: f32 = 0.003;
// How many rounds will averages change fast for
static ROUNDS_FINAL: f32 = 250.0;

// RWS Scale factor
// The base value we want our "average player" to sit at
static RWS_BASE: f32 = 10.0; // original would have been 20 
// value to use in our calculation (don't change this):
static RWS_SCALE: f32 = RWS_BASE * 5.0; // becomes 50 (halves the ammount of rws given from the original 100)

/**
 * Here we apply magic updates to a player's rws based on the previous round.
 */
// this assumes there are enough players in the game (non zero) to actually reward the player with score
pub fn calculate_rws(
    oldRws: f32,
    totalRounds: f32,
    wonRound: bool,
    roundPoints: f32,
    teamPoints: f32,
    teamPlayerCount: f32,
) -> Option<f32> {
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

    // run some safety checks to avoid overwriting users with a bad calculation

    // basically checking for zero
    if (teamPlayerCount < 0.9) {
      // if this is called with less than one player it was an erronous call
      println!("warning! calculate RWS was called with no players in teamPlayerCount.");
      // return NONE to avoid overwriting any existing values with a possibly erronous one
      return None;
    }

    if (teamPoints < roundPoints) {
      // The team total should never be less than the one users points
      println!("warning! calculate RWS was called with less team points than round points.");
      return None;
    }

    if (teamPoints > 0.0 && roundPoints > 0.0) {
        // scaled so it's always considered "out of 5 players" so different team sizes don't give inflated rws
        // If all 5 players do 100 dammage to 5 other players, we want to score them all as 10rws
        // 10 is where we want our base level of contribution to be set, but 100 / 5 is 20 it is at 20
        // old calculation: roundRws = 100.0 * teamPlayerCount / 5.0 * roundPoints / teamPoints;

        roundRws = RWS_SCALE * teamPlayerCount / 5.0 * roundPoints / teamPoints;

    } else {
      // if team or round points are zero,
      roundRws = 0.0;
    }
    if (!wonRound) {
        // if they didn't win, give them a quarter of their contribution points instead of nothing
        roundRws = roundRws * 0.25;
    }

    let alpha = GetAlphaFactor(totalRounds);

    //let newRws = (1.0 - alpha) * oldRws + alpha * roundRws;
    //newRws
   

    //println!("getting rws with alpha: {} old: {} rounds: {} won: {} roundpoints: {} teampoints: {} players: {}\nnew value: {} round rws: {}", alpha, oldRws, totalRounds, wonRound, roundPoints, teamPoints, teamPlayerCount, (1.0 - alpha) * oldRws + alpha * roundRws, roundRws);
    // Calculate the new rws average using the alpha factor to speed up changes at first
    Some((1.0 - alpha) * oldRws + alpha * roundRws)
}

fn GetAlphaFactor(rounds: f32) -> f32 {
    if (rounds < ROUNDS_FINAL) {
        return ALPHA_INIT + (ALPHA_INIT - ALPHA_FINAL) / (-ROUNDS_FINAL) * rounds;
    } else {
        return ALPHA_FINAL;
    }
}
