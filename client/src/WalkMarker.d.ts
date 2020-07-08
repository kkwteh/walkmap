import { Marker } from "leaflet";

export class WalkMarker extends Marker {
  textAnnotation: string;
  lat: number;
  lng: number;
}
