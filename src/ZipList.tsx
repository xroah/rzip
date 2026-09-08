import { useEffect, useState, MouseEvent } from "react"
import { Event, listen, TauriEvent } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core"
import BackArrow from "./icons/backArrow"
import { type Zip, format } from "./utils/zip"
import { Button } from "@base-ui/react/button"
import { ContextMenu } from "@base-ui/react/context-menu"
import clsx from "clsx"

interface Payload {
    paths: string[]
}

export default function ZipList() {
    const [zip, setZip] = useState<Zip>()
    const [stack, setStack] = useState<Zip[]>([])
    const [selected, setSelected] = useState<Set<number>>(new Set())
    const [currentCtxMenuItem, setCurrentCtxMenuItem] = useState<
        number | null
    >()
    const [ctxMenuOpen, setCtxMenuOpen] = useState(false)
    const handleCtxMenuOpen = (open: boolean) => {
        setCtxMenuOpen(open)

        if (!open) {
            setCurrentCtxMenuItem(null)
        }
    }
    const handleClick = (e: MouseEvent, idx: number, zip: Zip) => {
        e.stopPropagation()

        if (e.metaKey && e.shiftKey) {
            return
        }

        setSelected(s => {
            let id = zip.id

            let newSet = new Set(s)

            if (e.metaKey) {
                if (newSet.has(id)) {
                    newSet.delete(id)
                } else {
                    newSet.add(id)
                }
            } else if (e.shiftKey) {
            } else {
                if (newSet.has(id) && newSet.size === 1) {
                    newSet = new Set()
                } else {
                    return new Set([id])
                }
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

        setSelected(new Set([zip.id]))
    }
    const handleRootClick = () => {
        setSelected(new Set())
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

                            format(data, setZip)
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
        <div className="zip" onClick={handleRootClick}>
            <ul className="zip-header">
                <li className="zip-path zip-item">
                    <Button
                        onClick={handleBack}
                        disabled={!stack.length}
                        className="zip-back"
                    >
                        <BackArrow />
                    </Button>
                    <div className="zip-path-details">{zip?.path}</div>
                </li>
                <li className="zip-item">
                    <div className="file-name">Name</div>
                    <div className="file-last-modified">Date modified</div>
                    <div className="file-compressed-size">Compressed size</div>
                    <div className="file-size">Size</div>
                </li>
            </ul>
            <ContextMenu.Root
                open={ctxMenuOpen}
                defaultOpen={false}
                onOpenChange={handleCtxMenuOpen}
            >
                <ul className="zip-list">
                    {zip?.children.map((zip, i) => (
                        <ContextMenu.Trigger
                            key={zip.id}
                            render={
                                <li
                                    key={zip.id}
                                    onClick={e => handleClick(e, i, zip)}
                                    onDoubleClick={() => handleDoubleClick(zip)}
                                    className={clsx("zip-item", {
                                        "zip-item-selected": selected.has(
                                            zip.id,
                                        ),
                                        "zip-item-ctx-menu":
                                            currentCtxMenuItem === zip.id,
                                    })}
                                />
                            }
                            onContextMenu={e => {
                                setCurrentCtxMenuItem(zip.id)
                                console.log(e, "<<<<")
                            }}
                        >
                            {zip.isDir ? (
                                <div className="file-name">
                                    <span className="folder-default-icon zip-icon" />
                                    <span className="truncate" title={zip.name}>
                                        {zip.name}
                                    </span>
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
                            <div className="file-last-modified truncate">
                                {zip.modified}
                            </div>
                            <div className="file-compressed-size truncate">
                                {zip.compressedSize}
                            </div>
                            <div className="file-size truncate">{zip.size}</div>
                        </ContextMenu.Trigger>
                    ))}
                </ul>

                <ContextMenu.Portal>
                    <ContextMenu.Positioner sideOffset={4}>
                        <ContextMenu.Popup className="zip-ctx-menu rounded-[5px] p-[5px]">
                            <ContextMenu.Item className="zip-ctx-menu-item">
                                Open
                            </ContextMenu.Item>
                            <ContextMenu.Item className="zip-ctx-menu-item">
                                Rename
                            </ContextMenu.Item>
                            <ContextMenu.Item className="zip-ctx-menu-item">
                                Delete
                            </ContextMenu.Item>
                        </ContextMenu.Popup>
                    </ContextMenu.Positioner>
                </ContextMenu.Portal>
            </ContextMenu.Root>
        </div>
    )
}
