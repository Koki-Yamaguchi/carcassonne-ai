import axios from "axios";
import {
  Color,
  colorIDToColor,
  colorToColorID,
  idToTileKind,
  newTile,
} from "../tiles";
import {
  Game,
  MeepleMoveResult,
  CompleteEvent,
  Move,
  TileMove,
  MeepleMove,
  DiscardMove,
  Board,
  Player,
  WaitingGame,
  Problem,
  Vote,
  ProblemsResponse,
  ProblemProposal,
} from "../types";

export class API {
  base_url: string;
  constructor() {
    this.base_url = import.meta.env.VITE_API_BASE_URL;
  }

  async sendEvent(name: string, id: number) {
    try {
      const res = await axios.post(`${this.base_url}/send-event`, {
        name: name,
        id: Number(id),
      });
      console.log({ res });
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async subscribe(name: string, id: number, f: (event: any) => void) {
    try {
      const evtSource = new EventSource(
        `${this.base_url}/events?name=${name}&id=${id}`
      );
      evtSource.onmessage = f;
      return evtSource;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getPlayer(id: number): Promise<Player> {
    try {
      const res = await axios.get(`${this.base_url}/players/${id}`);
      console.log({ res });
      const p: Player = {
        id: res.data.id,
        name: res.data.name,
        userID: res.data.user_id,
        email: res.data.email,
        meepleColor: colorIDToColor(res.data.meeple_color),
        profileImageURL: res.data.profile_image_url,
        rating: res.data.rating,
      };
      return p;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getPlayerByUserID(userID: string): Promise<Player> {
    try {
      const res = await axios.get(`${this.base_url}/players?user=${userID}`);
      console.log({ res });
      const p: Player = {
        id: res.data.id,
        name: res.data.name,
        userID: res.data.user_id,
        email: res.data.email,
        meepleColor: colorIDToColor(res.data.meeple_color),
        profileImageURL: res.data.profile_image_url,
        rating: res.data.rating,
      };
      return p;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getPlayers(): Promise<Player[]> {
    try {
      const res = await axios.get(`${this.base_url}/players`);
      console.log({ res });
      const players = res.data.map((v: any) => {
        const player: Player = {
          id: v.id,
          name: v.name,
          userID: "",
          email: "",
          meepleColor: colorIDToColor(v.meeple_color),
          profileImageURL: v.profile_image_url,
          rating: v.rating,
        };
        return player;
      });
      return players;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async uploadProfileImage(playerID: number, data: any) {
    try {
      const res = await axios.post(
        `${this.base_url}/players/${playerID}/upload-profile-image`,
        data,
        {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        }
      );
      console.log({ res });
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async updatePlayer(
    id: number,
    name: string,
    meepleColor: number
  ): Promise<Player> {
    try {
      const res = await axios.post(`${this.base_url}/players/${id}/update`, {
        name,
        meeple_color: Number(meepleColor),
      });
      const player: Player = {
        id: res.data.id,
        name: res.data.name,
        email: res.data.email,
        userID: res.data.user_id,
        profileImageURL: res.data.profile_image_url,
        meepleColor: colorIDToColor(res.data.meeple_color),
      };
      return player;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createPlayer(
    name: string,
    email: string,
    userID: string
  ): Promise<Player> {
    try {
      const res = await axios.post(`${this.base_url}/players/create`, {
        name,
        email,
        user_id: userID,
      });
      const player: Player = {
        id: res.data.id,
        name: res.data.name,
        email: res.data.email,
        userID: res.data.user_id,
        profileImageURL: res.data.profile_image_url,
        meepleColor: colorIDToColor(res.data.meeple_color),
      };
      return player;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getWaitingGames(): Promise<WaitingGame[]> {
    try {
      const url = `${this.base_url}/waiting-games`;
      const res = await axios.get(url);
      console.log({ res });
      const games: WaitingGame[] = res.data.map((g: any) => {
        return {
          id: g.id,
          game_id: g.game_id,
          playerID: g.player_id,
        };
      });
      return games;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async updateWaitingGame(id: number, gameID: number): Promise<WaitingGame> {
    try {
      const res = await axios.post(
        `${this.base_url}/waiting-games/${id}/update`,
        {
          game_id: gameID,
        }
      );
      const waitingGame: WaitingGame = {
        id: res.data.id,
        game_id: res.data.game_id,
        playerID: res.data.player_id,
      };
      return waitingGame;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createWaitingGame(playerID: number): Promise<WaitingGame> {
    try {
      const res = await axios.post(`${this.base_url}/waiting-games/create`, {
        player_id: playerID,
      });
      const waitingGame: WaitingGame = {
        id: res.data.id,
        game_id: res.data.game_id,
        playerID: res.data.player_id,
      };
      return waitingGame;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async deleteWaitingGame(playerID: number) {
    try {
      await axios.post(`${this.base_url}/waiting-games/delete`, {
        player_id: playerID,
      });
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getGames(
    playerID: number | null,
    isRated: boolean,
    limit: number
  ): Promise<Game[]> {
    try {
      let url = `${this.base_url}/games?is_rated=${isRated}&limit=${limit}`;
      if (playerID) {
        url = `${url}&player=${playerID}`;
      }
      const res = await axios.get(url);
      console.log({ res });
      const games: Game[] = res.data.map((g: any) => {
        return {
          id: g.id,
          player0ID: g.player0_id,
          player1ID: g.player1_id,
          player0Point: g.player0_point,
          player1Point: g.player1_point,
          currentPlayerID: g.current_player_id,
          nextPlayerID: g.next_player_id,
          currentTileID: g.current_tile_id,
          nextTileID: g.next_tile_id,
          player0Name: g.player0_name,
          player1Name: g.player1_name,
          player0Color: colorIDToColor(g.player0_color),
          player1Color: colorIDToColor(g.player1_color),
          winnerPlayerID: g.winner_player_id,
          beforePlayer0Rating: g.before_player0_rating,
          afterPlayer0Rating: g.after_player0_rating,
          beforePlayer1Rating: g.before_player1_rating,
          afterPlayer1Rating: g.after_player1_rating,
          isRated: g.is_rated,
        };
      });
      return games;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getGame(id: number): Promise<Game> {
    try {
      const res = await axios.get(`${this.base_url}/games/${id}`);
      console.log({ res });
      const game: Game = {
        id: res.data.id,
        player0ID: res.data.player0_id,
        player1ID: res.data.player1_id,
        player0Point: res.data.player0_point,
        player1Point: res.data.player1_point,
        currentPlayerID: res.data.current_player_id,
        nextPlayerID: res.data.next_player_id,
        currentTileID: res.data.current_tile_id,
        nextTileID: res.data.next_tile_id,
        player0Name: res.data.player0_name,
        player1Name: res.data.player1_name,
        player0Color: colorIDToColor(res.data.player0_color),
        player1Color: colorIDToColor(res.data.player1_color),
        winnerPlayerID: res.data.winner_player_id,
        beforePlayer0Rating: res.data.before_player0_rating,
        afterPlayer0Rating: res.data.after_player0_rating,
        beforePlayer1Rating: res.data.before_player1_rating,
        afterPlayer1Rating: res.data.after_player1_rating,
        isRated: res.data.is_rated,
      };
      return game;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createGame(
    player0ID: number,
    player1ID: number,
    player0Color: Color,
    player1Color: Color,
    isRated: boolean
  ): Promise<Game> {
    try {
      const res = await axios.post(`${this.base_url}/games/create`, {
        player0_id: player0ID,
        player1_id: player1ID,
        player0_color: colorToColorID(player0Color),
        player1_color: colorToColorID(player1Color),
        is_rated: isRated,
      });
      const game: Game = {
        id: res.data.id,
        player0ID: res.data.player0_id,
        player1ID: res.data.player1_id,
        player0Point: res.data.player0_point,
        player1Point: res.data.player1_point,
        currentPlayerID: res.data.current_player_id,
        nextPlayerID: res.data.next_player_id,
        currentTileID: res.data.current_tile_id,
        nextTileID: res.data.next_tile_id,
        player0Name: res.data.player0_name,
        player1Name: res.data.player1_name,
        player0Color: colorIDToColor(res.data.player0_color),
        player1Color: colorIDToColor(res.data.player1_color),
        winnerPlayerID: res.data.winner_player_id,
        beforePlayer0Rating: res.data.before_player0_rating,
        afterPlayer0Rating: res.data.after_player0_rating,
        beforePlayer1Rating: res.data.before_player1_rating,
        afterPlayer1Rating: res.data.after_player1_rating,
        isRated: res.data.is_rated,
      };
      return game;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createTileMove(
    gameID: number | null,
    playerID: number,
    tileID: number,
    rot: number,
    posY: number,
    posX: number
  ): Promise<TileMove> {
    try {
      const res = await axios.post(`${this.base_url}/tile-moves/create`, {
        game_id: gameID,
        player_id: playerID,
        tile_id: tileID,
        rot: rot,
        pos_y: posY,
        pos_x: posX,
      });
      console.log({ res });
      const tileMove: TileMove = {
        id: res.data.TMove.id,
        playerID: res.data.TMove.player_id,
        ord: res.data.TMove.ord,
        tile: res.data.TMove.tile,
        pos: { y: res.data.TMove.pos[0], x: res.data.TMove.pos[1] },
        rot: res.data.TMove.rot,
      };
      return tileMove;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createMeepleMove(
    gameID: number | null,
    playerID: number,
    meepleID: number,
    pos: number,
    tilePosY: number,
    tilePosX: number
  ): Promise<MeepleMove> {
    try {
      const res = await axios.post(`${this.base_url}/meeple-moves/create`, {
        game_id: gameID,
        player_id: playerID,
        meeple_id: meepleID,
        pos: pos,
        tile_pos_y: tilePosY,
        tile_pos_x: tilePosX,
      });
      console.log({ res });

      const meepleMove: MeepleMove = {
        id: res.data.MMove.id,
        playerID: res.data.MMove.player_id,
        ord: res.data.MMove.ord,
        meepleID: res.data.MMove.meeple_id,
        pos: res.data.MMove.meeple_pos,
      };
      return meepleMove;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createDiscardMove(
    gameID: number,
    playerID: number,
    tileID: number
  ): Promise<DiscardMove> {
    try {
      const res = await axios.post(`${this.base_url}/discard-moves/create`, {
        game_id: gameID,
        player_id: playerID,
        tile_id: tileID,
      });
      console.log({ res });
      const discardMove: DiscardMove = {
        id: res.data.DMove.id,
        playerID: res.data.DMove.player_id,
        ord: res.data.DMove.ord,
        tile: res.data.DMove.tile,
      };
      return discardMove;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async waitAIMove(gameID: number) {
    try {
      const res = await axios.post(`${this.base_url}/wait-ai-move`, {
        game_id: gameID,
      });
      console.log({ res });
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getMoves(gameID: number, moveID?: number): Promise<Move[]> {
    try {
      const res = await axios.get(
        `${this.base_url}/moves?game=${gameID}` + (moveID ? `&m=${moveID}` : "")
      );
      console.log({ res });
      const moves = res.data.map((mv: any) => {
        if (mv.TMove) {
          const tm: TileMove = {
            id: mv.TMove.id,
            playerID: mv.TMove.player_id,
            ord: mv.TMove.ord,
            tile: mv.TMove.tile,
            pos: { y: mv.TMove.pos[0], x: mv.TMove.pos[1] },
            rot: mv.TMove.rot,
          };
          return tm;
        } else if (mv.MMove) {
          const mm: MeepleMove = {
            id: mv.MMove.id,
            playerID: mv.MMove.player_id,
            ord: mv.MMove.ord,
            meepleID: mv.MMove.meeple_id,
            pos: mv.MMove.meeple_pos,
          };
          return mm;
        } else {
          const dm: DiscardMove = {
            id: mv.DMove.id,
            playerID: mv.DMove.player_id,
            ord: mv.DMove.ord,
            tile: mv.DMove.tile,
          };
          return dm;
        }
      });
      return moves;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getFinalEevnts(gameID: number): Promise<MeepleMoveResult> {
    try {
      const res = await axios.get(
        `${this.base_url}/final-events?game=${gameID}`
      );
      console.log({ res });
      const meepleMoveResult: MeepleMoveResult = {
        completeEvents: res.data.complete_events.map(
          (e: any): CompleteEvent => {
            return {
              meepleIDs: e.meeple_ids,
              feature: e.feature,
              point: e.point,
            };
          }
        ),
        currentPlayerID: res.data.current_player_id,
        nextPlayerID: res.data.next_player_id,
        currentTileID: res.data.current_tile_id,
        nextTileID: res.data.next_tile_id,
      };
      return meepleMoveResult;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getBoard(
    gameID: number,
    player0MeepleColor: Color,
    player1MeepleColor: Color,
    moveID?: number
  ): Promise<Board> {
    try {
      const res = await axios.get(
        `${this.base_url}/board?game=${gameID}` + (moveID ? `&m=${moveID}` : "")
      );
      const board: Board = {
        player0Point: res.data.player0_point,
        player1Point: res.data.player1_point,
        meepleablePositions: res.data.meepleable_positions,
        tiles: res.data.tiles.map((row: any) => {
          return row.map((tile: any) => {
            const meepleColor =
              tile.meeple_id === -1
                ? null
                : tile.meeple_id < 7
                ? player0MeepleColor
                : player1MeepleColor;
            return tile.id === -1
              ? null
              : newTile(
                  tile.rot,
                  idToTileKind(tile.id),
                  meepleColor,
                  tile.meeple_pos,
                  tile.meeple_id
                );
          });
        }),
        completeEvents: res.data.complete_events.map(
          (e: any): CompleteEvent => {
            return {
              meepleIDs: e.meeple_ids,
              feature: e.feature,
              point: e.point,
            };
          }
        ),
      };
      return board;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getProblem(id: number): Promise<Problem> {
    try {
      const url = `${this.base_url}/problems/${id}`;
      const res = await axios.get(url);
      console.log({ res });
      const prob: Problem = {
        id: res.data.id,
        gameID: res.data.game_id,
        name: res.data.name,
        creatorID: res.data.creator_id,
        creatorName: res.data.creator_name,
        voteCount: res.data.vote_count,
      };
      return prob;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getProblems(
    page: number,
    orderBy: string,
    limit: number
  ): Promise<ProblemsResponse> {
    const params = new URLSearchParams();
    params.set("page", `${page}`);
    params.set("order_by", orderBy);
    params.set("limit", `${limit}`);
    try {
      const url = `${this.base_url}/problems?` + params.toString();
      const res = await axios.get(url);
      console.log({ res });
      const problems: Problem[] = res.data.problems.map((p: any) => {
        return {
          id: p.id,
          gameID: p.game_id,
          name: p.name,
          creatorID: p.creator_id,
          creatorName: p.creator_name,
          voteCount: p.vote_count,
        };
      });
      const totalCount = res.data.total_count;
      return {
        problems,
        totalCount,
      };
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createVote(
    problemID: number,
    playerID: number,
    playerName: string,
    note: string,
    tileMoveID: number,
    meepleMoveID: number
  ): Promise<Vote> {
    try {
      const res = await axios.post(`${this.base_url}/votes/create`, {
        problem_id: problemID,
        player_id: playerID,
        player_name: playerName,
        note: note,
        tile_move_id: tileMoveID,
        meeple_move_id: meepleMoveID,
      });
      const tileMove: TileMove = {
        id: res.data.tile_move.id,
        playerID: res.data.tile_move.player_id,
        ord: res.data.tile_move.ord,
        tile: res.data.tile_move.tile,
        pos: { y: res.data.tile_move.pos[0], x: res.data.tile_move.pos[1] },
        rot: res.data.tile_move.rot,
      };
      const meepleMove: MeepleMove = {
        id: res.data.meeple_move.id,
        playerID: res.data.meeple_move.player_id,
        ord: res.data.meeple_move.ord,
        meepleID: res.data.meeple_move.meeple_id,
        pos: res.data.meeple_move.meeple_pos,
      };
      const vote: Vote = {
        id: res.data.id,
        problemID: res.data.problem_id,
        problemName: res.data.problem_name,
        playerID: res.data.player_id,
        playerName: res.data.player_name,
        playerProfileImageURL: res.data.player_profile_image_url,
        note: res.data.note,
        tileMove,
        meepleMove,
        createdAt: new Date(res.data.created_at),
        lang: res.data.lang,
        translation: res.data.translation,
      };
      return vote;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getVotes(
    problemID: number | null,
    playerID: number | null
  ): Promise<Vote[]> {
    try {
      let url = `${this.base_url}/votes`;
      if (problemID) {
        url = `${url}?problem=${problemID}`;
      }
      if (playerID) {
        url = `${url}?player=${playerID}`;
      }
      const res = await axios.get(url);
      console.log({ res });
      const votes: Vote[] = res.data.map((v: any) => {
        let tileMove: TileMove | null = null;
        if (v.tile_move) {
          tileMove = {
            id: v.tile_move.id,
            playerID: v.tile_move.player_id,
            ord: v.tile_move.ord,
            tile: v.tile_move.tile,
            pos: { y: v.tile_move.pos[0], x: v.tile_move.pos[1] },
            rot: v.tile_move.rot,
          };
        }
        let meepleMove: MeepleMove | null = null;
        if (v.meeple_move) {
          meepleMove = {
            id: v.meeple_move.id,
            playerID: v.meeple_move.player_id,
            ord: v.meeple_move.ord,
            meepleID: v.meeple_move.meeple_id,
            pos: v.meeple_move.meeple_pos,
          };
        }
        return {
          id: v.id,
          problemID: v.problem_id,
          problemName: v.problem_name,
          playerID: v.player_id,
          playerName: v.player_name,
          playerProfileImageURL: v.player_profile_image_url,
          note: v.note,
          tileMove,
          meepleMove,
          createdAt: new Date(v.created_at),
          lang: v.lang,
          translation: v.translation,
        };
      });
      return votes;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createProblemProposal(
    tableID: number,
    remainingTileCount: number,
    creatorID: number,
    tileID: number
  ): Promise<ProblemProposal> {
    try {
      const res = await axios.post(
        `${this.base_url}/problem-proposals/create`,
        {
          table_id: tableID,
          remaining_tile_count: Number(remainingTileCount),
          creator_id: creatorID,
          tile_id: tileID,
        }
      );
      const proposal: ProblemProposal = {
        id: res.data.id,
        tableID: res.data.table_id,
        remainingTileCount: res.data.remaining_tile_count,
        creatorID: res.data.creator_id,
        tileID: res.data.tile_id,
      };
      return proposal;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }
}
