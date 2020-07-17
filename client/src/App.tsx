import React, { useState, useEffect } from "react";
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link,
  Redirect,
  useParams,
  useRouteMatch,
  withRouter,
} from "react-router-dom";
import axios from "axios";
import { WalkMarker } from "./WalkMarker";
import { MainMap, mapCache } from "./MainMap";
import { Sidebar } from "./Sidebar";
import { Marker } from "leaflet";
import "./App.css";
import "leaflet/dist/leaflet.css";

// Add react-router-dom
// Redirect
// await insert new map
// redirect to /:map_id

function App() {
  return (
    <Router>
      <Switch>
        <Route path="/:mapId">
          <Map />
        </Route>
        <Route path="/">
          <Init />
        </Route>
      </Switch>
    </Router>
  );
}

function Init() {
  const [mapId, setMapId] = useState<string | undefined>();
  console.log("mapId", mapId);
  // change to async await
  // useEffect(async () => {
  //   const result = await axios(
  //     'https://hn.algolia.com/api/v1/search?query=redux',
  //   );

  //   setData(result.data);
  // });
  let config = {
    headers: { "Access-Control-Allow-Origin": "*" },
  };
  useEffect(() => {
    // axios
    //   .get(
    //     "https://gist.githubusercontent.com/witalewski/fc8f043d53a0d505f84c5ddb04ae76ea/raw/7c505bbc1675a0bc8a067f8b633b531c769bb64c/data.json"
    //   )
    //   .then(({ data }) => {
    //     setMapId("ABCDEF");
    //   });
    axios.post("http://localhost:8081/map", config).then(({ data }) => {
      console.log("Data", data);
      setMapId("ABCDEF");
    });
  }, []);

  if (mapId === undefined) {
    return <div>Fetching map information</div>;
  }

  return <Redirect to={`/${mapId}`} />;
}

function Map() {
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
