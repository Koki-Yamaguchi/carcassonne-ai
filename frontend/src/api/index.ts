import axios from "axios";
import {
  Game,
  MeepleMoveResult,
  CompleteEvent,
  TileMoveResult,
  Move,
  TileMove,
  MeepleMove,
} from "../types";

export class API {
  base_url: string;
  constructor() {
    this.base_url = "http://127.0.0.1:8000";
  }

  async getGames() {
    try {
      const res = await axios.get(`${this.base_url}/games?player=1`);
      const games = res.data;
      return games;
    } catch (e) {
      console.log({ e });
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
        nextPlayerID: res.data.next_player_id,
        nextTileID: res.data.next_tile_id,
      };
      return game;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async createGame(player0ID: number, player1ID: number): Promise<Game> {
    try {
      const res = await axios.post(`${this.base_url}/games/create`, {
        player0_id: player0ID,
        player1_id: player1ID,
        note: "",
      });
      const game: Game = {
        id: res.data.id,
        player0ID: res.data.player0_id,
        player1ID: res.data.player1_id,
        nextPlayerID: res.data.next_player_id,
        nextTileID: res.data.next_tile_id,
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
        nextPlayerID: res.data.next_player_id,
        nextTileID: res.data.next_tile_id,
      };
      return meepleMoveResult;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async waitAIMove(gameID: number): Promise<MeepleMoveResult> {
    try {
      const res = await axios.post(`${this.base_url}/wait-ai-move`, {
        game_id: gameID,
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
        nextPlayerID: res.data.next_player_id,
        nextTileID: res.data.next_tile_id,
      };
      return meepleMoveResult;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }

  async getMoves(gameID: number): Promise<Move[]> {
    try {
      const res = await axios.get(`${this.base_url}/moves?game=${gameID}`);
      console.log({ res });
      const moves = res.data.map((mv: any, idx: number) => {
        if (idx % 2 === 0) {
          const tm: TileMove = {
            ord: mv.TMove.ord,
            tile: mv.TMove.tile,
            pos: { y: mv.TMove.pos[0], x: mv.TMove.pos[1] },
            rot: mv.TMove.rot,
          };
          return tm;
        } else {
          const mm: MeepleMove = {
            ord: mv.MMove.ord,
            meepleID: mv.MMove.meeple_id,
            pos: mv.MMove.meeple_pos,
          };
          return mm;
        }
      });
      return moves;
    } catch (e) {
      console.log({ e });
      throw e;
    }
  }
}
