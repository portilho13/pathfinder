import React from "react";
import { useState } from "react";
import "./Icons.css";

interface IconsProps {
    onSelectIcon: (icon: string) => void;
    selectedAlgorithm: (algorith: string) => void;
  }

const Icons = ({ onSelectIcon,  selectedAlgorithm} : IconsProps) => {
    const [selectedOption, setselectedOption] = React.useState<string>("dijkstra");
    const handleSelectOption = (e: React.ChangeEvent<HTMLSelectElement>) => {
        setselectedOption(e.target.value);
        selectedAlgorithm(e.target.value);
    }

    const handleIconClick = (icon: string) => {
        if (icon === "start") {
            onSelectIcon("start");
        } else if (icon === "destination") {
            onSelectIcon("destination");
        } else if (icon === "busStop") {
            onSelectIcon("busStop");
        } 
    };



    return (
        <>
            <div className="icons-container">
                <select className="dropdown" onChange={handleSelectOption} value={selectedOption}>
                    <option value="dijkstra">Dijkstra Algorithm</option>
                    <option value="aStar">A*</option>
                </select>
                <div className="icon" onClick={() => handleIconClick("start")}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-cursor-fill" viewBox="0 0 16 16">
                        <path d="M14.082 2.182a.5.5 0 0 1 .103.557L8.528 15.467a.5.5 0 0 1-.917-.007L5.57 10.694.803 8.652a.5.5 0 0 1-.006-.916l12.728-5.657a.5.5 0 0 1 .556.103z"/>
                    </svg>
                    <label className="icon-label">Start</label>
                </div>
                <div className="icon" onClick={() => handleIconClick("destination")}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-geo-alt-fill" viewBox="0 0 16 16">
                        <path d="M8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10m0-7a3 3 0 1 1 0-6 3 3 0 0 1 0 6"/>
                    </svg>
                    <label className="icon-label">Destination</label>
                </div>
                <div className="icon" onClick={() => handleIconClick("busStop")}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-bus-front-fill" viewBox="0 0 16 16">
                        <path d="M16 7a1 1 0 0 1-1 1v3.5c0 .818-.393 1.544-1 2v2a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5V14H5v1.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-2a2.5 2.5 0 0 1-1-2V8a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1V2.64C1 1.452 1.845.408 3.064.268A44 44 0 0 1 8 0c2.1 0 3.792.136 4.936.268C14.155.408 15 1.452 15 2.64V4a1 1 0 0 1 1 1zM3.552 3.22A43 43 0 0 1 8 3c1.837 0 3.353.107 4.448.22a.5.5 0 0 0 .104-.994A44 44 0 0 0 8 2c-1.876 0-3.426.109-4.552.226a.5.5 0 1 0 .104.994M8 4c-1.876 0-3.426.109-4.552.226A.5.5 0 0 0 3 4.723v3.554a.5.5 0 0 0 .448.497C4.574 8.891 6.124 9 8 9s3.426-.109 4.552-.226A.5.5 0 0 0 13 8.277V4.723a.5.5 0 0 0-.448-.497A44 44 0 0 0 8 4m-3 7a1 1 0 1 0-2 0 1 1 0 0 0 2 0m8 0a1 1 0 1 0-2 0 1 1 0 0 0 2 0m-7 0a1 1 0 0 0 1 1h2a1 1 0 1 0 0-2H7a1 1 0 0 0-1 1"/>
                    </svg>
                    <label className="icon-label">Bus Stop</label>
                </div>
            </div>
        </>
    );
}
export default Icons;