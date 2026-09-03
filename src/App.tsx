import { useEffect } from "react"
import "./App.css"
import { invoke } from "@tauri-apps/api/core"
import { Event, listen, TauriEvent } from "@tauri-apps/api/event"
interface Payload {
    paths: string[]
}

declare global {
    interface Window {
        invokeTauri: typeof invoke
    }
}

window.invokeTauri = invoke

function App() {
    useEffect(() => {
        const listenPromise = listen(
            TauriEvent.DRAG_DROP,
            (event: Event<Payload>) => {
                const { paths } = event.payload

                if (paths.length) {
                    invoke("get_zip_json", { file: paths[0] })
                        .then(ret => {
                            const data = JSON.parse(ret as string)

                            console.log(data)
                        })
                        .catch(e => {
                            console.error(e)
                        })
                }
            },
        )

        return () => {
            listenPromise.then(rm => rm())
        }
    }, [])

    return <main className="container"></main>
}

export default App
