import { TileKind, Tile, Color } from "./tiles";

export interface Player {
  id: number;
  name: string;
  email: string;
  userID: string;
  meepleColor: Color;
  profileImageURL: string;
  rating?: number;
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
  winnerPlayerID: number;
  beforePlayer0Rating: number;
  afterPlayer0Rating: number;
  beforePlayer1Rating: number;
  afterPlayer1Rating: number;
  isRated: boolean;
}

export interface WaitingGame {
  id: number;
  game_id?: number;
  playerID: number;
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

export interface DiscardMove {
  playerID: number;
  ord: number;
  tile: TileKind;
}

export type Move = TileMove | MeepleMove | DiscardMove;

export interface Board {
  player0Point: number;
  player1Point: number;
  tiles: Tile[][];
  meepleablePositions: number[];
  completeEvents: CompleteEvent[];
}

export interface Problem {
  id: number;
  gameID: number;
  name: string;
}

export interface Vote {
  id: number;
  problemID: number;
  playerID: number;
  playerName: string;
  note: string;
  tileMoveID: number;
  meepleMoveID: number;
}
