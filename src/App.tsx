import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import fileLogo from "./assets/document.png";
import dirLogo from "./assets/folder.png";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [directory, setDirectory] = useState([]);
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
    //let resultOfReadDir = await invoke("read_dir");
    //console.log(resultOfReadDir);
  }

  useEffect(() => {
    invoke("read_dir").then((result) => {
      setDirectory(result);
    });
  }, [directory]);

  return (
    <div className="container">
      <h1>Pascal File Compressor</h1>

      {directory.map((file) => (
        <div>
          <span>
          {file.is_dir ? (
            <img src={dirLogo}  />
          ): (
            <img src={fileLogo} />
            )}
          </span>
          <p>{file.name}</p>
        </div>
      ))}
    </div>
  )

 // return (
 //   <div className="container">
 //     <h1>Welcome to Tauri!</h1>

 //     <form
 //       className="row"
 //       onSubmit={(e) => {
 //         e.preventDefault();
 //         greet();
 //       }}
 //     >
 //       <input
 //         id="greet-input"
 //         onChange={(e) => setName(e.currentTarget.value)}
 //         placeholder="Enter a name..."
 //       />
 //       <button type="submit">Greet</button>
 //     </form>

 //     <p>{greetMsg}</p>
 //   </div>
 // );
}

export default App;
