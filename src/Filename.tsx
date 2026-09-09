import { Input } from "@base-ui/react/input"
import {
    ChangeEvent,
    MouseEvent,
    KeyboardEvent,
    useEffect,
    useRef,
    useState,
} from "react"

interface Props {
    name: string
    path: string
    isDir: boolean
    icon?: string
    isEditing?: boolean
    onRename?: (newName: string, path: string) => void
    onDone?: VoidFunction
}

interface NameRage {
    hasExt: boolean
    range: number[]
}

function getNameRange(name: string) {
    let index = name.lastIndexOf(".")
    name = name.trim().replace(/^[\s\.]*/, "")
    let ret: NameRage = {
        hasExt: true,
        range: [],
    }

    if (!name.includes(".")) {
        ret.hasExt = false
    } else {
        ret.range = [0, index]
    }

    return ret
}

export default function Filename({
    name,
    path,
    isDir,
    icon,
    isEditing = false,
    onRename,
    onDone,
}: Props) {
    const stopPropagation = (e: MouseEvent) => {
        e.stopPropagation()
    }
    const [newName, setNewName] = useState(name)
    const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
        setNewName(e.target.value)
    }
    const inputRef = useRef<HTMLInputElement>(null)
    const handleKeydown = (e: KeyboardEvent) => {
        const k = e.key.toLowerCase()

        if (k === "enter") {
            onDone?.()
        } else if (k === "esc" || k === "escape") {
            setNewName(name)
            onDone?.()
        }
    }

    useEffect(() => {
        if (!isEditing) {
            if (name !== newName) {
                onRename?.(newName, path)
                console.log("rrrr", newName)
            }
        } else {
            let input = inputRef.current

            if (!input) {
                return
            }

            input.focus()
            if (isDir) {
                input.select()
            } else {
                let ret = getNameRange(name)

                if (ret.hasExt) {
                    input.setSelectionRange(
                        ret.range[0],
                        ret.range[1],
                        "forward",
                    )
                } else {
                    input.select()
                }
            }
        }
    }, [isEditing])

    return (
        <div className="file-name">
            {isDir ? (
                <span className="folder-default-icon zip-icon" />
            ) : (
                <span
                    className="file-default-icon zip-icon"
                    style={{
                        backgroundImage: icon
                            ? `url('/icons/${icon}.svg')`
                            : undefined,
                    }}
                />
            )}

            {isEditing ? (
                <Input
                    className="px-[5px] text-gray-700 rounded border border-solid border-gray-300 h-[24px]"
                    onContextMenu={stopPropagation}
                    onClick={stopPropagation}
                    onDoubleClick={stopPropagation}
                    value={newName}
                    onChange={handleChange}
                    onKeyDown={handleKeydown}
                    ref={inputRef}
                />
            ) : (
                <span className="truncate" title={name}>
                    {name}
                </span>
            )}
        </div>
    )
}
