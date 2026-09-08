import { invoke } from "@tauri-apps/api/core"

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

export interface Zip {
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

let manifestCache: Manifest | null = null

function formatSize(s: number) {
    let ret = ""
    const BASE = 1024
    const units = [ "B", "KB", "MB", "GB" ]
    const len = units.length
    let i = 0

    while (i < len) {
        if (s < BASE) {
            ret = s.toFixed(2)
            break
        }

        s = s / BASE
        
        i++
    }

    return ret + units[ i ]
}

let id = 100

function getIcon(
    { icon_map, code_map }: Manifest,
    { ext, name, is_dir }: ArchiveNode,
) {
    if (is_dir) {
        return
    }

    let icon = icon_map[ name ]

    if (!icon && ext) {
        icon = icon_map[ ext ]
    }

    return icon ? code_map[ icon ] : undefined
}

function sort(zip1: Zip, zip2: Zip) {
    return zip1.name > zip2.name ? 1 : zip1.name < zip2.name ? -1 : 0
}

export async function format(archiveNode: ArchiveNode, cb: (zip: Zip) => void) {
    if (!manifestCache) {
        manifestCache = await invoke("get_icon_manifest")
    }
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
            let node = children[ k ]
            let zipNode: Zip = {
                name: node.name,
                isDir: true,
                size: formatSize(node.size),
                compressedSize: formatSize(node.compressed_size),
                icon: getIcon(manifestCache!, node),
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

        return [ ...folders.sort(sort), ...files.sort(sort) ]
    }

    ret.children = fmt(archiveNode, ret)

    cb(ret)
    console.log(ret)
}