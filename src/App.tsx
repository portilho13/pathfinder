import React, { useState } from "react";
import "./App.css";
import HeaderComponent from "./components/headerComponent/headerComponent";
import Grid from "./components/Grid/Grid";
import Icons from "./components/Icons/Icons";

const App: React.FC = () => {
  const [selectedIcon, setSelectedIcon] = useState<string>("");
  const [selectedAlgorithm, setSelectedAlgorithm] = useState<string>("dijkstra");

  const handleAlgorithmSelect = (algorithm: string) => {
    setSelectedAlgorithm(algorithm);
    console.log(`Algorithm selected: ${algorithm}`);
  }

  const handleCellClick = (x: number, y: number) => {
    console.log(`Cell clicked at ${x}, ${y}`);
  };

  const handleSelectIcon = (icon: string) => {
    setSelectedIcon(icon);
  };

  return (
    <div className="container">
      <HeaderComponent />
      <Icons onSelectIcon={handleSelectIcon} selectedAlgorithm={handleAlgorithmSelect}/>
      <Grid
        onCellClick={handleCellClick}
        icon={selectedIcon}
        algorithm={selectedAlgorithm}
      />
    </div>
  );
};

export default App;
