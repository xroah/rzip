import "./styles/tailwind.css";
import "./styles/index.scss"
import { invoke } from "@tauri-apps/api/core"
import ZipList from "./ZipList"

declare global {
    interface Window {
        invokeTauri: typeof invoke
    }
}

window.invokeTauri = invoke

function App() {
    return (
        <main className="container">
            <ZipList />
        </main>
    )
}

export default App
