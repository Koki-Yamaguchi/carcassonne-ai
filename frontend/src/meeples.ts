import { Color } from "./tiles";
import RedStanding from "./assets/img/red_standing.png";
import YellowStanding from "./assets/img/yellow_standing.png";
import GreenStanding from "./assets/img/green_standing.png";
import BlackStanding from "./assets/img/black_standing.png";
import BlueStanding from "./assets/img/blue_standing.png";
import RedLying from "./assets/img/red_lying.png";
import YellowLying from "./assets/img/yellow_lying.png";
import GreenLying from "./assets/img/green_lying.png";
import BlackLying from "./assets/img/black_lying.png";
import BlueLying from "./assets/img/blue_lying.png";

function standingMeepleSrc(color: Color): any {
  if (color === "red") {
    return RedStanding;
  }
  if (color === "yellow") {
    return YellowStanding;
  }
  if (color === "green") {
    return GreenStanding;
  }
  if (color === "black") {
    return BlackStanding;
  }
  if (color === "blue") {
    return BlueStanding;
  }
  return null;
}

function lyingMeepleSrc(color: Color): any {
  if (color === "red") {
    return RedLying;
  }
  if (color === "yellow") {
    return YellowLying;
  }
  if (color === "green") {
    return GreenLying;
  }
  if (color === "black") {
    return BlackLying;
  }
  if (color === "blue") {
    return BlueLying;
  }
  return null;
}

export { standingMeepleSrc, lyingMeepleSrc };
