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
  TileMoveResult,
  Move,
  TileMove,
  MeepleMove,
  DiscardMove,
  Board,
  Player,
} from "../types";

export class API {
  base_url: string;
  constructor() {
    this.base_url = import.meta.env.VITE_API_BASE_URL;
  }

  async sendEvent(gameID: number) {
    try {
      const res = await axios.post(`${this.base_url}/send-event`, {
        game_id: Number(gameID),
      });
      console.log({ res });
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async join(gid: number, f: (event: any) => void) {
    try {
      const evtSource = new EventSource(`${this.base_url}/events?game=${gid}`);
      evtSource.onmessage = f;
      return evtSource;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getPlayer(userID: string): Promise<Player> {
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
      };
      return game;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createTileMove(
    gameID: number,
    playerID: number,
    tileID: number,
    rot: number,
    posY: number,
    posX: number
  ): Promise<TileMoveResult> {
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
      const tileMoveResult: TileMoveResult = {
        meepleablePositions: res.data.meepleable_positions,
      };
      return tileMoveResult;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createMeepleMove(
    gameID: number,
    playerID: number,
    meepleID: number,
    pos: number,
    tilePosY: number,
    tilePosX: number
  ): Promise<MeepleMoveResult> {
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

  async createDiscardMove(
    gameID: number,
    playerID: number,
    tileID: number
  ): Promise<MeepleMoveResult> {
    try {
      const res = await axios.post(`${this.base_url}/discard-moves/create`, {
        game_id: gameID,
        player_id: playerID,
        tile_id: tileID,
      });
      console.log({ res });
      const meepleMoveResult: MeepleMoveResult = {
        completeEvents: [],
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
            playerID: mv.TMove.player_id,
            ord: mv.TMove.ord,
            tile: mv.TMove.tile,
            pos: { y: mv.TMove.pos[0], x: mv.TMove.pos[1] },
            rot: mv.TMove.rot,
          };
          return tm;
        } else if (mv.MMove) {
          const mm: MeepleMove = {
            playerID: mv.MMove.player_id,
            ord: mv.MMove.ord,
            meepleID: mv.MMove.meeple_id,
            pos: mv.MMove.meeple_pos,
          };
          return mm;
        } else {
          const dm: DiscardMove = {
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
}
