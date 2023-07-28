import StartingTile from "./assets/img/city_cap_with_straight.png";
import Monastery from "./assets/img/monastery.png";
import MonasteryWithRoad from "./assets/img/monastery_with_road.png";
import Straight from "./assets/img/straight.png";
import Curve from "./assets/img/curve.png";
import QuadrupleRoad from "./assets/img/quadruple_road.png";
import TripleRoad from "./assets/img/triple_road.png";
import Triangle from "./assets/img/triangle.png";
import TriangleWithCOA from "./assets/img/triangle_with_coa.png";
import TriangleWithRoad from "./assets/img/triangle_with_road.png";
import TriangleWithRoadWithCOA from "./assets/img/triangle_with_road_with_coa.png";
import CityCapWithCrossroad from "./assets/img/city_cap_with_crossroads.png";
import CityCap from "./assets/img/city_cap.png";
import Left from "./assets/img/left.png";
import Right from "./assets/img/right.png";
import Connector from "./assets/img/connector.png";
import ConnectorWithCOA from "./assets/img/connector_with_coa.png";
import VerticalSeparator from "./assets/img/vertical_separator.png";
import TripleCity from "./assets/img/triple_city.png";
import TripleCityWithCOA from "./assets/img/triple_city_with_coa.png";
import TripleCityWithRoad from "./assets/img/triple_city_with_road.png";
import TripleCityWithRoadWithCOA from "./assets/img/triple_city_with_road_with_coa.png";
import QuadrupleCityWithCOA from "./assets/img/quadruple_city_with_coa.png";
import Separator from "./assets/img/separator.png";

export type TileKind =
  | "StartingTile"
  | "Monastery"
  | "MonasteryWithRoad"
  | "CityCapWithCrossroad"
  | "TriangleWithRoad"
  | "TriangleWithRoadWithCOA"
  | "Straight"
  | "CityCap"
  | "Separator"
  | "TripleRoad"
  | "Curve"
  | "QuadrupleRoad"
  | "Connector"
  | "ConnectorWithCOA"
  | "Left"
  | "Right"
  | "TripleCity"
  | "TripleCityWithCOA"
  | "VerticalSeparator"
  | "TripleCityWithRoad"
  | "TripleCityWithRoadWithCOA"
  | "Triangle"
  | "TriangleWithCOA"
  | "QuadrupleCityWithCOA";

type Side = "field" | "road" | "city";

export type Color = "red" | "yellow" | "green" | "black" | "blue" | null;

export const colorIDToColor = (colorID: number): Color => {
  if (colorID === 0) return "red";
  if (colorID === 1) return "yellow";
  if (colorID === 2) return "green";
  if (colorID === 3) return "black";
  if (colorID === 4) return "blue";
  return null;
};

export type Position = {
  idx: number;
  y: number;
  x: number;
  isField: boolean;
};

export class Tile {
  direction: number;
  src: any;
  defaultMeepleablePositions: Position[] = [];
  sides: Side[];
  meepleID = -1;
  meepledPosition = -1;
  meepleColor: Color;
  frame: Color = null;
  right(): Side {
    return this.sides[(0 + this.direction) % 4];
  }
  top(): Side {
    return this.sides[(1 + this.direction) % 4];
  }
  left(): Side {
    return this.sides[(2 + this.direction) % 4];
  }
  bottom(): Side {
    return this.sides[(3 + this.direction) % 4];
  }
  rotate() {
    this.direction = (this.direction + 1) % 4;
  }
  resetDirection() {
    this.direction = 0;
  }
  placeMeeple(idx: number, color: Color, meepleID: number) {
    this.meepledPosition = idx;
    this.meepleColor = color;
    this.meepleID = meepleID;
  }
  removeMeeple() {
    this.meepledPosition = -1;
    this.meepleColor = null;
    this.meepleID = -1;
  }
  addFrame(color: Color) {
    this.frame = color;
  }
  meepleablePositions(emptyPositions: number[]): Position[] {
    return this.defaultMeepleablePositions
      .filter((pos) => {
        return emptyPositions.includes(pos.idx);
      })
      .map((pos) => {
        const y = pos.y;
        const x = pos.x;
        const theta = -Math.PI * 0.5 * this.direction;
        const toY = x * Math.sin(theta) + y * Math.cos(theta);
        const toX = x * Math.cos(theta) - y * Math.sin(theta);
        return { idx: pos.idx, y: toY, x: toX, isField: pos.isField };
      });
  }
  constructor(
    direction: number,
    sides: Side[],
    src: any,
    meepleColor: Color,
    meepledPostion: number,
    meepleID: number,
    defaultMeepleablePositions?: Position[],
    frame?: Color
  ) {
    this.direction = direction;
    this.sides = sides;
    this.src = src;
    this.meepleColor = meepleColor;
    this.meepledPosition = meepledPostion;
    this.meepleID = meepleID;
    if (defaultMeepleablePositions) {
      this.defaultMeepleablePositions = defaultMeepleablePositions;
    }
    if (frame) {
      this.frame = frame;
    }
  }
}

export function newTile(
  rot: number,
  tileKind: TileKind,
  meepleColor: Color,
  meepledPosition: number,
  meepleID: number
): Tile {
  return new Tile(
    rot,
    getSides(tileKind),
    getSrc(tileKind),
    meepleColor,
    meepledPosition,
    meepleID,
    getDefaultMeeplePositions(tileKind)
  );
}

function getSides(tileKind: TileKind): Side[] {
  switch (tileKind) {
    case "StartingTile": {
      return ["road", "city", "road", "field"];
    }
    case "Monastery": {
      return ["field", "field", "field", "field"];
    }
    case "MonasteryWithRoad": {
      return ["field", "field", "field", "road"];
    }
    case "CityCapWithCrossroad": {
      return ["road", "city", "road", "road"];
    }
    case "TriangleWithRoad":
    case "TriangleWithRoadWithCOA": {
      return ["road", "city", "city", "road"];
    }
    case "Straight": {
      return ["field", "road", "field", "road"];
    }
    case "CityCap": {
      return ["field", "city", "field", "field"];
    }
    case "Separator": {
      return ["field", "city", "city", "field"];
    }
    case "TripleRoad": {
      return ["road", "field", "road", "road"];
    }
    case "Curve": {
      return ["field", "field", "road", "road"];
    }
    case "QuadrupleRoad": {
      return ["road", "road", "road", "road"];
    }
    case "Connector":
    case "ConnectorWithCOA": {
      return ["city", "field", "city", "field"];
    }
    case "Left": {
      return ["field", "city", "road", "road"];
    }
    case "Right": {
      return ["road", "city", "field", "road"];
    }
    case "TripleCity":
    case "TripleCityWithCOA": {
      return ["city", "city", "city", "field"];
    }
    case "VerticalSeparator": {
      return ["field", "city", "field", "city"];
    }
    case "TripleCityWithRoad":
    case "TripleCityWithRoadWithCOA": {
      return ["city", "city", "city", "road"];
    }
    case "Triangle":
    case "TriangleWithCOA": {
      return ["field", "city", "city", "field"];
    }
    case "QuadrupleCityWithCOA": {
      return ["city", "city", "city", "city"];
    }
    default: {
      return [];
    }
  }
}

function getSrc(tileKind: TileKind): any {
  switch (tileKind) {
    case "StartingTile": {
      return StartingTile;
    }
    case "Monastery": {
      return Monastery;
    }
    case "MonasteryWithRoad": {
      return MonasteryWithRoad;
    }
    case "CityCapWithCrossroad": {
      return CityCapWithCrossroad;
    }
    case "TriangleWithRoad": {
      return TriangleWithRoad;
    }
    case "TriangleWithRoadWithCOA": {
      return TriangleWithRoadWithCOA;
    }
    case "Straight": {
      return Straight;
    }
    case "CityCap": {
      return CityCap;
    }
    case "Separator": {
      return Separator;
    }
    case "TripleRoad": {
      return TripleRoad;
    }
    case "Curve": {
      return Curve;
    }
    case "QuadrupleRoad": {
      return QuadrupleRoad;
    }
    case "Connector": {
      return Connector;
    }
    case "ConnectorWithCOA": {
      return ConnectorWithCOA;
    }
    case "Left": {
      return Left;
    }
    case "Right": {
      return Right;
    }
    case "TripleCity": {
      return TripleCity;
    }
    case "TripleCityWithCOA": {
      return TripleCityWithCOA;
    }
    case "VerticalSeparator": {
      return VerticalSeparator;
    }
    case "TripleCityWithRoad": {
      return TripleCityWithRoad;
    }
    case "TripleCityWithRoadWithCOA": {
      return TripleCityWithRoadWithCOA;
    }
    case "Triangle": {
      return Triangle;
    }
    case "TriangleWithCOA": {
      return TriangleWithCOA;
    }
    case "QuadrupleCityWithCOA": {
      return QuadrupleCityWithCOA;
    }
    default: {
      return null;
    }
  }
}

function getDefaultMeeplePositions(tileKind: TileKind): Position[] {
  switch (tileKind) {
    case "StartingTile": {
      return [
        { idx: 0, y: 0.8, x: 0, isField: false },
        { idx: 1, y: 0.3, x: 0.7, isField: true },
        { idx: 2, y: 0, x: 0, isField: false },
        { idx: 3, y: -0.6, x: 0, isField: true },
      ];
    }
    case "Monastery": {
      return [
        { idx: 0, y: 0, x: 0, isField: false },
        { idx: 1, y: 0.6, x: 0.6, isField: true },
      ];
    }
    case "MonasteryWithRoad": {
      return [
        { idx: 0, y: 0, x: 0, isField: false },
        { idx: 1, y: 0.6, x: 0.6, isField: true },
        { idx: 2, y: -0.75, x: 0, isField: false },
      ];
    }
    case "CityCapWithCrossroad": {
      return [
        { idx: 0, y: 0.8, x: 0, isField: false },
        { idx: 1, y: 0.3, x: 0.8, isField: true },
        { idx: 2, y: -0.1, x: -0.6, isField: false },
        { idx: 3, y: -0.2, x: 0.6, isField: false },
        { idx: 4, y: -0.6, x: -0.7, isField: true },
        { idx: 5, y: -0.6, x: -0.1, isField: false },
        { idx: 6, y: -0.6, x: 0.7, isField: true },
      ];
    }
    case "TriangleWithRoad":
    case "TriangleWithRoadWithCOA": {
      return [
        { idx: 0, y: 0.5, x: -0.5, isField: false },
        { idx: 1, y: -0.1, x: 0.1, isField: true },
        { idx: 2, y: -0.4, x: 0.4, isField: false },
        { idx: 3, y: -0.7, x: 0.7, isField: true },
      ];
    }
    case "Straight": {
      return [
        { idx: 0, y: 0, x: -0.5, isField: true },
        { idx: 1, y: 0, x: 0, isField: false },
        { idx: 2, y: 0, x: 0.5, isField: true },
      ];
    }
    case "CityCap": {
      return [
        { idx: 0, y: 0.8, x: 0, isField: false },
        { idx: 1, y: -0.1, x: 0, isField: true },
      ];
    }
    case "Separator": {
      return [
        { idx: 0, y: 0.8, x: 0, isField: false },
        { idx: 1, y: 0, x: -0.85, isField: false },
        { idx: 2, y: -0.4, x: 0.4, isField: true },
      ];
    }
    case "TripleRoad": {
      return [
        { idx: 0, y: 0.7, x: 0, isField: true },
        { idx: 1, y: 0.1, x: -0.7, isField: false },
        { idx: 2, y: 0.1, x: 0.7, isField: false },
        { idx: 3, y: -0.5, x: -0.5, isField: true },
        { idx: 4, y: -0.5, x: 0, isField: false },
        { idx: 5, y: -0.5, x: 0.5, isField: true },
      ];
    }
    case "Curve": {
      return [
        { idx: 0, y: 0.5, x: 0.5, isField: true },
        { idx: 1, y: 0, x: 0, isField: false },
        { idx: 2, y: -0.5, x: -0.5, isField: true },
      ];
    }
    case "QuadrupleRoad": {
      return [
        { idx: 0, y: 0.5, x: -0.5, isField: true },
        { idx: 1, y: 0.7, x: 0.1, isField: false },
        { idx: 2, y: 0.5, x: 0.5, isField: true },
        { idx: 3, y: 0, x: -0.7, isField: false },
        { idx: 4, y: -0.1, x: 0.7, isField: false },
        { idx: 5, y: -0.5, x: -0.5, isField: true },
        { idx: 6, y: -0.7, x: 0, isField: false },
        { idx: 7, y: -0.5, x: 0.5, isField: true },
      ];
    }
    case "Connector":
    case "ConnectorWithCOA": {
      return [
        { idx: 0, y: 0.85, x: 0, isField: true },
        { idx: 1, y: 0, x: 0, isField: false },
        { idx: 2, y: -0.8, x: 0, isField: true },
      ];
    }
    case "Left": {
      return [
        { idx: 0, y: 0.8, x: 0, isField: false },
        { idx: 1, y: -0.1, x: 0.5, isField: true },
        { idx: 2, y: -0.25, x: -0.25, isField: false },
        { idx: 3, y: -0.6, x: -0.6, isField: true },
      ];
    }
    case "Right": {
      return [
        { idx: 0, y: 0.8, x: 0, isField: false },
        { idx: 1, y: -0.1, x: -0.5, isField: true },
        { idx: 2, y: -0.25, x: 0.25, isField: false },
        { idx: 3, y: -0.6, x: 0.6, isField: true },
      ];
    }
    case "TripleCity":
    case "TripleCityWithCOA": {
      return [
        { idx: 0, y: 0.1, x: 0, isField: false },
        { idx: 1, y: -0.7, x: 0, isField: true },
      ];
    }
    case "VerticalSeparator": {
      return [
        { idx: 0, y: 0.8, x: 0, isField: false },
        { idx: 1, y: 0, x: 0, isField: true },
        { idx: 2, y: -0.8, x: 0, isField: false },
      ];
    }
    case "TripleCityWithRoad":
    case "TripleCityWithRoadWithCOA": {
      return [
        { idx: 0, y: 0.1, x: 0, isField: false },
        { idx: 1, y: -0.8, x: -0.4, isField: true },
        { idx: 2, y: -0.7, x: 0, isField: false },
        { idx: 3, y: -0.8, x: 0.4, isField: true },
      ];
    }
    case "Triangle":
    case "TriangleWithCOA": {
      return [
        { idx: 0, y: 0.5, x: -0.5, isField: false },
        { idx: 1, y: -0.4, x: 0.4, isField: true },
      ];
    }
    case "QuadrupleCityWithCOA": {
      return [{ idx: 0, y: 0, x: 0, isField: false }];
    }
    default: {
      return [];
    }
  }
}

export function idToTileKind(id: number): TileKind {
  switch (id) {
    case 0: {
      return "StartingTile";
    }
    case 1: {
      return "Monastery";
    }
    case 2: {
      return "MonasteryWithRoad";
    }
    case 3: {
      return "CityCapWithCrossroad";
    }
    case 4: {
      return "TriangleWithRoad";
    }
    case 5: {
      return "TriangleWithRoadWithCOA";
    }
    case 6: {
      return "Straight";
    }
    case 7: {
      return "CityCap";
    }
    case 8: {
      return "Separator";
    }
    case 9: {
      return "TripleRoad";
    }
    case 10: {
      return "Curve";
    }
    case 11: {
      return "QuadrupleRoad";
    }
    case 12: {
      return "Connector";
    }
    case 13: {
      return "ConnectorWithCOA";
    }
    case 14: {
      return "Left";
    }
    case 15: {
      return "Right";
    }
    case 16: {
      return "TripleCity";
    }
    case 17: {
      return "TripleCityWithCOA";
    }
    case 18: {
      return "VerticalSeparator";
    }
    case 19: {
      return "TripleCityWithRoad";
    }
    case 20: {
      return "TripleCityWithRoadWithCOA";
    }
    case 21: {
      return "Triangle";
    }
    case 22: {
      return "TriangleWithCOA";
    }
    case 23: {
      return "QuadrupleCityWithCOA";
    }
    default: {
      return "QuadrupleCityWithCOA";
    }
  }
}

export const boardSize = 2 * 20 + 1;
export function getInitialBoard(): (Tile | null)[][] {
  const initialBoard: (Tile | null)[][] = [];
  for (let i = 0; i < boardSize; i++) {
    const emptyRow: (Tile | null)[] = [];
    for (let j = 0; j < boardSize; j++) {
      emptyRow.push(null);
    }
    initialBoard.push(emptyRow);
  }
  initialBoard[(boardSize - 1) / 2][(boardSize - 1) / 2] = newTile(
    0,
    "StartingTile",
    null,
    -1,
    -1
  );
  return initialBoard;
}
