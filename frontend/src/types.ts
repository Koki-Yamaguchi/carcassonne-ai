import { TileKind } from "./tiles";

export interface Game {
  id: number;
  player0ID: number;
  player1ID: number;
  nextPlayerID: number;
  nextTileID: number;
}

export interface TilePosition {
  y: number;
  x: number;
}

export interface TileMoveResult {
  meepleablePositions: number[];
}

export interface MeepleMoveResult {
  completeEvents: CompleteEvent[];
  nextTileID: number;
  nextPlayerID: number;
}

export interface CompleteEvent {
  meepleIDs: number[];
  feature: string;
  point: number;
}

export interface TileMove {
  ord: number;
  tile: TileKind;
  pos: TilePosition;
  rot: number;
}

export interface MeepleMove {
  ord: number;
  meepleID: number;
  pos: number;
}

export type Move = TileMove | MeepleMove;
