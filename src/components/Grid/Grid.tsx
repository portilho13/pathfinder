import React, { useState } from 'react';
import './Grid.css';
import { invoke } from '@tauri-apps/api/tauri';
interface GridProps {
  onCellClick: (x: number, y: number) => void;
  icon: string;
  algorithm: string;
}

interface Cell {
  x: number;
  y: number;
  isStart: boolean;
  isFinish: boolean;
  isVisited: boolean;
  isMarked: boolean;
}

const Grid: React.FC<GridProps> = ({ onCellClick, icon, algorithm}) => {
  // Initialize state to manage the grid of cells
  const [grid, setGrid] = useState<Cell[][]>(initializeGrid());

  // Function to initialize the grid with default values
  function initializeGrid(): Cell[][] {
    const newGrid: Cell[][] = [];
    for (let x = 0; x < 50; x++) {
      const row: Cell[] = [];
      for (let y = 0; y < 20; y++) {
        row.push({ x, y, isVisited: false, isStart: false, isFinish: false, isMarked: false});
      }
      newGrid.push(row);
    }
    return newGrid;
  }

  const handleCellClick = (x: number, y: number) => {
    onCellClick(x, y);
    setGrid(prevGrid => {
      const updatedGrid = prevGrid.map(row =>
        row.map(cell => {
          if (cell.x === x && cell.y === y) {
            // Toggle start cell
            if (icon === 'start') {
              resetStart(x, y);
              return { ...cell, isStart: !cell.isStart, isFinish: false };
            }
            // Toggle finish cell
            else if (icon === 'destination') {
              resetFinish(x, y);
              return { ...cell, isFinish: !cell.isFinish, isStart: false };
            }
          }
          return cell;
        })
      );
      return updatedGrid;
    });
  };


  const resetStart = (x: number, y: number) => {
    setGrid(prevGrid => {
      const updatedGrid = prevGrid.map(row =>
        row.map(cell => {
          if (cell.x === x && cell.y === y) {
            // Don't reset the clicked cell
            return cell;
          }
          if (cell.isStart) {
            return { ...cell, isStart: false };
          }
          return cell;
        })
      );
      return updatedGrid;
    });
  };

  const resetFinish = (x: number, y: number) => {
    setGrid(prevGrid => {
      const updatedGrid = prevGrid.map(row =>
        row.map(cell => {
          if (cell.x === x && cell.y === y) {
            // Don't reset the clicked cell
            return cell;
          }
          if (cell.isFinish) {
            return { ...cell, isFinish: false };
          }
          return cell;
        })
      );
      return updatedGrid;
    });
  };

  // Render the grid of cells
  const renderGrid = () => {
    return grid.map((row, rowIndex) => (
      <div key={rowIndex} className="row">
        {row.map((cell, cellIndex) => (
          <div
            key={`${cell.x}-${cell.y}`}
            className={'cell'}
            onClick={() => handleCellClick(cell.x, cell.y)}
          >
            {cell.isStart && (
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-cursor-fill" viewBox="0 0 16 16">
                <path d="M14.082 2.182a.5.5 0 0 1 .103.557L8.528 15.467a.5.5 0 0 1-.917-.007L5.57 10.694.803 8.652a.5.5 0 0 1-.006-.916l12.728-5.657a.5.5 0 0 1 .556.103z"/>
              </svg>
            )}
            {cell.isFinish && (
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-geo-alt-fill" viewBox="0 0 16 16">
                <path d="M8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10m0-7a3 3 0 1 1 0-6 3 3 0 0 1 0 6"/>
              </svg>
            )}
          </div>
        ))}
      </div>
    ));
  };

  async function generatePath() {
    let start: Cell | undefined;
    let finish: Cell | undefined;
  
    // Find the start and finish cells
    grid.forEach(row => {
      row.forEach(cell => {
        if (cell.isStart) {
          start = cell;
        }
        if (cell.isFinish) {
          finish = cell;
        }
      });
    });
  
    if (!start || !finish) {
      // Handle the case where either the start or finish cell is not found
      console.error("Start or finish cell not found");
      return;
    }
    // console.log(`Generating path from ${start.x}, ${start.y} to ${finish.x}, ${finish.y}`);  
    // await invoke('choose_algorithm', {
    //   algorithm: algorithm,
    //   start_x: start.x,
    //   start_y: start.y,
    //   destination_x: finish.x,
    //   destination_y: finish.y
    // });
    // console.log(`Generating path from ${start.x}, ${start.y} to ${finish.x}, ${finish.y}`);
    const algoGrid: number[][] = [];

    grid.forEach(row => {
      const newRow: number[] = [];
      row.forEach(cell => {
        if (cell.isMarked) {
          newRow.push(0);
        } else {
          newRow.push(1);
        }
      });
      algoGrid.push(newRow);
    });
    await invoke('choose_algorithm', {algorithm: algorithm, grid: algoGrid});
  };
  

  return (
      <>
          <div className="grid">{renderGrid()}</div>
          <div className="button-container">
            <button className="button" onClick={generatePath}>
              Generate
            </button>
          </div>
      </>
    );
};

export default Grid;
