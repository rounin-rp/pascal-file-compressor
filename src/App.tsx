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
  is_compressed: boolean;
}

function App() {
  const [directory, setDirectory] = useState([]);
  const [selectedFile, setSelectedFile] = useState<Directory>(null);
  const [selectedFileIndex, setSelectedFileIndex] = useState(-1);

  const clearSelection = () => {
    setSelectedFile(null);
    setSelectedFileIndex(-1);
  }

  const onClickHandler = async (currDir: Directory, currIndex: number) => {
    if(currDir.is_dir){
        clearSelection();
        invoke("plugin:read_dir|click_dir", {selectedDir: currDir}).then((result) => {
        setDirectory(result);
      });
    }else{
      if(selectedFileIndex === currIndex){
        clearSelection();
      }else{
        setSelectedFileIndex(currIndex);
        setSelectedFile(currDir);
      }
    }
  }

  const backClickHandler = () => {
    setSelectedFile(null);
    setSelectedFileIndex(-1);
    invoke("plugin:read_dir|back_dir").then((result) => {
      setDirectory(result);
    });
  }

  const compressClickHandler = () => {
      invoke("plugin:compression|compress_file", {currFile: selectedFile}).then((result) => {
      invoke("plugin:read_dir|read_dir").then((result) => {
        setDirectory(result);
      })
    })
  }

  useEffect(() => {
    invoke("plugin:read_dir|read_dir").then((result) => {
      setDirectory(result);
    });
  }, [directory]);

   
  return (
    <div className="container">
      <h1>Pascal File Compressor</h1>
      <div className="backButton">
        <button onClick={backClickHandler}>Back</button>
      </div>
      {directory.map((file, index) => (
        <div key={index} className="fileDisplay" style={selectedFileIndex === index ? {"border": "2px solid blue"}: {}} onClick={() => onClickHandler(file, index)}>
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
      <div className="compressedButton">
       {(selectedFile && !selectedFile.is_compressed) ? (
        <button onClick={compressClickHandler}>Compress</button>   
        ):(selectedFile) ? (
          <button>Decompress</button>
        ): (<></>)
       } 
      </div>
    </div>
  )
}

export default App;
