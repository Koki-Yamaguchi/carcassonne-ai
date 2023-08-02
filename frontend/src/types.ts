import { TileKind, Tile, Color } from "./tiles";

export interface Player {
  id: number;
  name: string;
  email: string;
  userID: string;
  meepleColor: Color;
}

export interface Game {
  id: number;
  player0ID: number;
  player1ID: number;
  player0Name: string;
  player1Name: string;
  player0Color: Color;
  player1Color: Color;
  player0Point: number;
  player1Point: number;
  currentPlayerID: number;
  nextPlayerID: number;
  currentTileID: number;
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
  currentTileID: number;
  nextTileID: number;
  currentPlayerID: number;
  nextPlayerID: number;
}

export interface CompleteEvent {
  meepleIDs: number[];
  feature: string;
  point: number;
}

export interface TileMove {
  playerID: number;
  ord: number;
  tile: TileKind;
  pos: TilePosition;
  rot: number;
}

export interface MeepleMove {
  playerID: number;
  ord: number;
  meepleID: number;
  pos: number;
}

export type Move = TileMove | MeepleMove;

export interface Board {
  player0Point: number;
  player1Point: number;
  tiles: Tile[][];
  meepleablePositions: number[];
}
