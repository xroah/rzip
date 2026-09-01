import {useEffect} from "react";
import "./App.css";
import {invoke} from "@tauri-apps/api/core"

function App() {
    useEffect(() => {
        // @ts-ignore
        window.getZipJson = (file: string) => {
            return invoke("get_json", {file})
        }
    }, [])

    return (
        <main className="container">
        </main>
    );
}

export default App;
