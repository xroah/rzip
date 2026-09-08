import { useEffect, useState, MouseEvent } from "react"
import { Event, listen, TauriEvent } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core"
import BackArrow from "./icons/backArrow"

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
    full_path: string
}

interface Zip {
    name: string
    isDir: boolean
    modified?: string
    size: string
    compressedSize: string
    icon?: string
    children: Zip[]
    id: number
    parent?: Zip
    path: string
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

let id = 100

function getIcon(
    { icon_map, code_map }: Manifest,
    { ext, name, is_dir }: ArchiveNode,
) {
    if (is_dir) {
        return
    }

    let icon = icon_map[name]

    if (!icon && ext) {
        icon = icon_map[ext]
    }

    return icon ? code_map[icon] : undefined
}

function sort(zip1: Zip, zip2: Zip) {
    return zip1.name > zip2.name ? 1 : zip1.name < zip2.name ? -1 : 0
}

async function format(archiveNode: ArchiveNode, cb: (zip: Zip) => void) {
    let manifest = (await invoke("get_icon_manifest")) as Manifest
    let ret: Zip = {
        name: "root",
        isDir: true,
        size: "",
        compressedSize: "",
        icon: "",
        modified: "",
        id: 0,
        children: [],
        path: "/",
    }
    const fmt = (an: ArchiveNode, parent?: Zip) => {
        let folders: Zip[] = []
        let files: Zip[] = []
        let children = an.children
        let keys = Object.keys(children)

        for (let k of keys) {
            let node = children[k]
            let zipNode: Zip = {
                name: node.name,
                isDir: true,
                size: formatSize(node.size),
                compressedSize: formatSize(node.compressed_size),
                icon: getIcon(manifest, node),
                modified: node.last_modified,
                id: id++,
                children: [],
                parent,
                path: node.full_path,
            }

            if (node.is_dir) {
                folders.push(zipNode)
                zipNode.children = fmt(node, zipNode)
            } else {
                zipNode.isDir = false

                files.push(zipNode)
            }
        }

        return [...folders.sort(sort), ...files.sort(sort)]
    }

    ret.children = fmt(archiveNode, ret)

    cb(ret)
    console.log(ret)
}

export default function ZipList() {
    const [zip, setZip] = useState<Zip>()
    const [stack, setStack] = useState<Zip[]>([])
    const [selected, setSelected] = useState<Set<number>>(new Set())
    const handleClick = (e: MouseEvent, zip: Zip) => {
        setSelected(s => {
            if (!e.ctrlKey) {
                return new Set([zip.id])
            }

            let newSet = new Set(s)

            if (newSet.has(zip.id)) {
                newSet.delete(zip.id)
            } else {
                newSet.add(zip.id)
            }

            return newSet
        })
    }
    const handleDoubleClick = (zip: Zip) => {
        if (zip.isDir) {
            setStack(stack => [...stack, zip.parent!])
            setZip(zip)
            setSelected(new Set())

            return
        }
    }
    const handleBack = () => {
        let current = stack.pop()

        setZip(current)
        setStack([...stack])
        setSelected(new Set([]))
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

                            format(data as ArchiveNode, setZip)
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
        <div className="zip">
            <ul className="zip-header">
                <li className="zip-path zip-item">
                    <button
                        onClick={handleBack}
                        disabled={!stack.length}
                        className="zip-back"
                    >
                        <BackArrow />
                    </button>
                    <input
                        readOnly
                        className="zip-path-input"
                        value={zip?.path}
                    />
                </li>
                <li className="zip-item">
                    <div className="file-name">Name</div>
                    <div className="file-last-modified">Date modified</div>
                    <div className="file-compressed-size">Compressed size</div>
                    <div className="file-size">Size</div>
                </li>
            </ul>
            <ul className="zip-list">
                {zip?.children.map(zip => (
                    <li
                        key={zip.id}
                        onClick={e => handleClick(e, zip)}
                        onDoubleClick={() => handleDoubleClick(zip)}
                        className={`zip-item ${selected.has(zip.id) ? "zip-item-selected" : ""}`}
                    >
                        {zip.isDir ? (
                            <div className="file-name">
                                <span className="folder-default-icon zip-icon" />
                                {zip.name}
                            </div>
                        ) : (
                            <div className="file-name">
                                <span
                                    className="file-default-icon zip-icon"
                                    style={{
                                        backgroundImage: zip.icon
                                            ? `url('/icons/${zip.icon}.svg')`
                                            : undefined,
                                    }}
                                />
                                <span className="truncate" title={zip.name}>
                                    {zip.name}
                                </span>
                            </div>
                        )}
                    </li>
                ))}
            </ul>
        </div>
    )
}
