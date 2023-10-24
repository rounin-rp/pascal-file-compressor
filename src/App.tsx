import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import fileLogo from "./assets/document.png";
import dirLogo from "./assets/folder.png";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface Directory {
  name: string;
  path: string;
  is_dir: boolean;
}

function App() {
  const [directory, setDirectory] = useState([]);

  const onClickHandler = async (currDir: Directory) => {
    console.log(currDir);
    if(currDir.is_dir){
      invoke("click_dir", {selectedDir: currDir}).then((result) => {
        setDirectory(result);
      });
    }
  }

  const backClickHandler = () => {
    invoke("back_dir").then((result) => {
      setDirectory(result);
    });
  }

  useEffect(() => {
    invoke("read_dir").then((result) => {
      setDirectory(result);
    });
  }, [directory]);

  return (
    <div className="container">
      <h1>Pascal File Compressor</h1>
      <div className="backButton">
        <button onClick={backClickHandler}>Back</button>
      </div>
      {directory.map((file) => (
        <div className="fileDisplay" onClick={() => onClickHandler(file)}>
          <span>
          {file.is_dir ? (
            <img src={dirLogo} style={{"height": "50px", "width": "50px"}} />
          ):(
            <img src={fileLogo} style={{"height": "50px", "width": "50px"}} />
            )}
          </span>
          <p>{file.name}</p>
        </div>
      ))}
    </div>
  )
}

export default App;
