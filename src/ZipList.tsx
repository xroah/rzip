import { useEffect, useState } from "react"
import { Event, listen, TauriEvent } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core"

interface Payload {
    paths: string[]
}

interface ArchiveNode {
    name: string
    is_dir: boolean
    last_modified?: string
    size: number
    compressed_size: number
    ext?: string
    children: Record<string, ArchiveNode>
}

interface Zip {
    name: string
    isDir: boolean
    modified?: string
    size: string
    compressedSize: string
    icon?: string
    children?: Zip[]
    key: number
}

type IconCodeMap = Record<number, string>
type IconMap = Record<string, number>

interface Manifest {
    icon_map: IconMap
    code_map: IconCodeMap
}

function formatSize(s: number) {
    let ret = ""
    const BASE = 1024
    const units = ["B", "KB", "MB", "GB"]
    const len = units.length
    let i = 0

    while (i < len) {
        if (s < BASE) {
            ret = s.toFixed(2)
            break
        }

        s = s / BASE
    }

    return ret + units[i]
}

let key = 0

function getIcon(
    { icon_map, code_map }: Manifest,
    { ext, is_dir, name }: ArchiveNode,
) {
    let defaultIcon = is_dir ? "folder" : "file"
    let icon = icon_map[name]

    if (!icon && ext) {
        icon = icon_map[ext]
    }

    return icon ? code_map[icon] || defaultIcon : defaultIcon
}

async function format(archiveNode: ArchiveNode, cb: (zip: Zip[]) => void) {
    let manifest = (await invoke("get_icon_manifest")) as Manifest

    let ret: Zip[] = []
    const fmt = (an: typeof archiveNode.children) => {
        let folders: Zip[] = []
        let files: Zip[] = []
        let keys = Object.keys(an)

        for (let k of keys) {
            let node = an[k]
            let zipNode: Zip = {
                name: node.name,
                isDir: true,
                size: formatSize(node.size),
                compressedSize: formatSize(node.compressed_size),
                icon: getIcon(manifest, node),
                modified: node.last_modified,
                key: key++,
            }

            if (node.is_dir) {
                folders.push(zipNode)
                zipNode.children = fmt(node.children)
            } else {
                zipNode.isDir = false

                files.push(zipNode)
            }
        }

        return [...folders, ...files]
    }

    ret = fmt(archiveNode.children)

    cb(ret)
    console.log(ret)
}

export default function ZipList() {
    const [zipList, setZipList] = useState<Zip[]>([])
    const [stack, setStack] = useState<Zip[][]>([])
    const handleClick = (zip: Zip) => {}
    const handleDoubleClick = (zip: Zip) => {
        if (zip.isDir) {
            setStack(stack => [...stack, zipList])
            setZipList(zip.children!)
            return
        }
    }
    const handleBack = () => {
        let current = stack.pop()

        setZipList(current!)
        setStack([...stack])
    }

    useEffect(() => {
        const listenPromise = listen(
            TauriEvent.DRAG_DROP,
            (event: Event<Payload>) => {
                const { paths } = event.payload

                if (paths.length) {
                    invoke("get_zip_json", { file: paths[0] })
                        .then(ret => {
                            const data = JSON.parse(ret as string)

                            format(data as ArchiveNode, setZipList)
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

    return (
        <ul className="zip-list">
            <li className="zip-list-header">
                <div className="file-name">Name</div>
                <div className="file-last-modified">Date modified</div>
                <div className="file-compressed-size">Compressed size</div>
                <div className="file-size">Size</div>
            </li>
            {!!stack.length && (
                <li>
                    <button
                        onClick={handleBack}
                        className="rounded bg-blue-500 text-white px-[10px] py-[5px] cursor-pointer hover:bg-blue-400 active:bg-blue-600 transition-colors"
                    >
                        Back
                    </button>
                </li>
            )}
            {zipList?.map(zip => {
                let bg = zip.icon ? `url('/icons/${zip.icon}.svg')` : undefined

                return (
                    <li
                        key={zip.key}
                        onClick={() => handleClick(zip)}
                        onDoubleClick={() => handleDoubleClick(zip)}
                        className="zip-item"
                    >
                        {zip.isDir ? (
                            <div className="file-name">
                                <span
                                    className="folder-default-icon zip-icon"
                                    style={{
                                        backgroundImage: bg,
                                    }}
                                />
                                {zip.name}
                            </div>
                        ) : (
                            <div className="file-name">
                                <span
                                    className="file-default-icon zip-icon"
                                    style={{ backgroundImage: bg }}
                                />
                                {zip.name}
                            </div>
                        )}
                    </li>
                )
            })}
        </ul>
    )
}
