import React, { useState, ChangeEvent, MouseEvent } from "react";
import { WalkMarker } from "./WalkMarker";

type SidebarProps = {
  selectedMarker: WalkMarker | undefined;
  setMarkerText: (text: string) => void;
  nextMarker: () => void;
  prevMarker: () => void;
  deleteSelectedMarker: () => void;
};

export function Sidebar(props: SidebarProps) {
  const [textData, updateTextData] = useState<string | undefined>(
    props.selectedMarker?.textAnnotation
  );
  const [isEditMode, setIsEditMode] = useState<boolean>(
    props.selectedMarker?.textAnnotation !== undefined
  );

  if (props.selectedMarker === undefined) {
    return <div className="Sidebar">sidebar</div>;
  }

  const handleChange = (event: ChangeEvent) => {
    // @ts-ignore
    updateTextData(event?.currentTarget?.value);
  };

  const handleSave = (event: MouseEvent) => {
    if (textData === undefined) {
      return;
    }
    props.setMarkerText(textData);
    setIsEditMode(false);
  };

  const handleCancel = (event: MouseEvent) => {
    setIsEditMode(false);
  };

  const handleEditClick = (event: MouseEvent) => {
    setIsEditMode(true);
  };

  const textAnnotation = isEditMode ? null : (
    <div>
      <div>{props.selectedMarker?.textAnnotation}</div>
      <div className="button" onClick={handleEditClick}>
        Edit Text
      </div>
    </div>
  );

  const textForm = isEditMode ? (
    <div>
      <textarea
        onChange={handleChange}
        defaultValue={props.selectedMarker?.textAnnotation || ""}
      ></textarea>
      <div className="button" onClick={handleSave}>
        Save
      </div>
      <div className="button" onClick={handleCancel}>
        Cancel
      </div>
    </div>
  ) : null;

  return (
    <div className="Sidebar">
      <div>
        {props.selectedMarker.lat}, {props.selectedMarker.lng}
      </div>
      {textAnnotation}
      {textForm}
      <div className="button" onClick={props.prevMarker}>
        Prev
      </div>
      <div className="button" onClick={props.nextMarker}>
        Next
      </div>
      <div className="button" onClick={props.deleteSelectedMarker}>
        Delete
      </div>
    </div>
  );
}
