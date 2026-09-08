import {MouseEvent} from "react"
import { invoke } from "@tauri-apps/api/core"
import ZipList from "./ZipList"

import "./styles/tailwind.css"
import "./styles/index.scss"

declare global {
    interface Window {
        invokeTauri: typeof invoke
    }
}

window.invokeTauri = invoke

function App() {
    const preventCtxMenu = (e: MouseEvent) => {
        if (!import.meta.env.DEV) {
            e.preventDefault()
        }
    }

    return (
        <main onContextMenu={preventCtxMenu}>
            <ZipList />
        </main>
    )
}

export default App
