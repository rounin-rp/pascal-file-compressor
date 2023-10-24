import { useState, useEffect } from "react";
import fileLogo from "./assets/document.png";
import dirLogo from "./assets/folder.png";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface CurrFile {
  name: string;
  path: string;
  is_dir: boolean;
  is_compressed: boolean;
}

interface CurrDir {
  name: string;
  path: string;
}

function App() {
  const [currFile, setCurrFile] = useState<CurrFile[]>([]);
  const [selectedFile, setSelectedFile] = useState<CurrFile|null>(null);
  const [currDir, setCurrDir] = useState<CurrDir|null>(null);
  const [selectedFileIndex, setSelectedFileIndex] = useState(-1);

  const clearSelection = () => {
    setSelectedFile(null);
    setSelectedFileIndex(-1);
  }

  const readDirectory = async () => {
      const [currentFile, currentDir]: [CurrFile[], CurrDir] = await invoke("plugin:read_dir|read_dir", {currDir: currDir});
      if(currentDir !== currDir) {
      setCurrDir(currentDir);
      setCurrFile(currentFile);
      }
  }

  const clickDirectory = async (currDir: CurrFile) => {
    clearSelection();
    const [currentFile, currentDir]: [CurrFile[], CurrDir] = await invoke("plugin:read_dir|click_dir", {selectedDir: currDir});
    if (currentDir !== currDir){
      setCurrDir(currentDir);
      setCurrFile(currentFile);
    }
  }

  const onClickHandler = async (currDir: CurrFile, currIndex: number) => {
    if(currDir.is_dir){
      await clickDirectory(currDir);
    }else{
      if(selectedFileIndex === currIndex){
        clearSelection();
      }else{
        setSelectedFileIndex(currIndex);
        setSelectedFile(currDir);
      }
    }
  }

  const backClickHandler = async () => {
    clearSelection();
    const [currentFile, currentDir]: [CurrFile[], CurrDir] = await invoke("plugin:read_dir|back_dir", {currDir: currDir});
    if(currentDir !== currDir) {
    setCurrDir(currentDir);
    setCurrFile(currentFile);
    }
  }

  const compressClickHandler = async () => {
    console.log("compress");
    await invoke("plugin:compression|compress_file", {currFile: selectedFile});
    await readDirectory();
  }

  const decompressClickHandler = async () => {
    await invoke("plugin:compression|extract_file", {currFile: selectedFile});     
    await readDirectory();
  }

  useEffect(() => {
    const invoke_read_dir = async () => {
      await readDirectory();
    }
    invoke_read_dir();
  }, [currFile]);
  
    
  return (
    <div className="container">
      <h1>Pascal File Compressor</h1>
      <div className="backButton">
        <button onClick={backClickHandler}>Back</button>
      </div>
      {currFile.map((file, index) => (
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
          <button onClick={decompressClickHandler}>Decompress</button>
        ): (<></>)
       } 
      </div>
    </div>
  )
}

export default App;
