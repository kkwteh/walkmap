import React, { useState } from "react";
import "./App.css";
import { WalkMarker } from "./WalkMarker";
import { MainMap, mapCache } from "./MainMap";
import { Sidebar } from "./Sidebar";
import { Marker } from "leaflet";
import "leaflet/dist/leaflet.css";

function App() {
  const [selectedMarker, setSelectedMarker] = useState<WalkMarker | undefined>(
    undefined
  );
  const [markers, setMarkers] = useState<WalkMarker[]>([]);

  const findSelectedMarkerIndex = () => {
    if (selectedMarker === undefined) {
      return -1;
    }

    let selectedMarkerIndex: number = -1;
    markers.forEach((marker, index) => {
      if (
        marker.lat === ((selectedMarker as unknown) as WalkMarker).lat &&
        marker.lng === ((selectedMarker as unknown) as WalkMarker).lng
      ) {
        selectedMarkerIndex = index;
      }
    });
    return selectedMarkerIndex;
  };

  const deleteSelectedMarker = () => {
    let map = mapCache.map;
    if (map === undefined) {
      return;
    }

    map.removeLayer(selectedMarker as Marker);
    const selectedMarkerIndex: number = findSelectedMarkerIndex();
    if (selectedMarkerIndex !== -1) {
      markers.splice(selectedMarkerIndex, 1);
      setMarkers(markers);
    }
    // TODO: select a new selectedMarker and update state
  };

  const prevMarker = () => {
    if (selectedMarker === undefined) {
      return;
    }
    let selectedIndex: number = findSelectedMarkerIndex();

    if (selectedIndex > 0) {
      setSelectedMarker(markers[selectedIndex - 1]);
    }
  };

  const nextMarker = () => {
    if (selectedMarker === undefined) {
      return;
    }
    let selectedIndex: number = findSelectedMarkerIndex();

    if (-1 < selectedIndex && selectedIndex < markers.length - 1) {
      setSelectedMarker(markers[selectedIndex + 1]);
    }
  };

  const pushMarker = (marker: WalkMarker) => {
    for (let existing_marker of markers as WalkMarker[]) {
      if (
        existing_marker.lat === marker.lat &&
        existing_marker.lng === marker.lng
      ) {
        return;
      }
    }
    (markers as WalkMarker[]).push(marker);
    setMarkers(markers);
    setSelectedMarker(marker);
  };

  const setMarkerText = (text: string) => {
    if (selectedMarker === undefined) {
      throw new Error(
        "selectMarker cannot be undefined when calling setMarkerText"
      );
    }
    selectedMarker.textAnnotation = text;
  };
  console.log("markers", markers);
  console.log("map", mapCache.map);
  return (
    <div
      className="App"
      style={{
        height: "600px",
        width: "100vw",
      }}
    >
      <Sidebar
        setMarkerText={setMarkerText}
        selectedMarker={selectedMarker}
        nextMarker={nextMarker}
        prevMarker={prevMarker}
        deleteSelectedMarker={deleteSelectedMarker}
      ></Sidebar>
      <MainMap
        selectedMarker={selectedMarker}
        setSelectedMarker={setSelectedMarker}
        pushMarker={pushMarker}
      ></MainMap>
    </div>
  );
}

export default App;
